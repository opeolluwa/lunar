use std::sync::Arc;

use lunar::entities::workspace_members;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    adapters::{jwt::Claims, workspace_member::AccountWorkspaceResponse},
    errors::app_error::AppError,
    repositories::{
        base::Repository,
        workspace_member::{WorkspaceMemberRepository, WorkspaceMemberRepositoryTrait},
    },
};

pub const ROLE_OWNER: &str = "owner";
pub const ROLE_ADMIN: &str = "admin";
pub const ROLE_MEMBER: &str = "member";

#[derive(Clone)]
pub struct WorkspaceMemberService {
    member_repository: WorkspaceMemberRepository,
}

impl WorkspaceMemberService {
    pub fn new(member_repository: WorkspaceMemberRepository) -> Self {
        Self {
            member_repository,
        }
    }

    pub fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            member_repository: WorkspaceMemberRepository::init(db_conn),
        }
    }

    /// Ensure the given account holds a membership row for this workspace.
    /// The first account to claim a workspace becomes its owner; later claims
    /// by unknown accounts are rejected.
    pub async fn ensure_owner(&self, workspace_identifier: Uuid, claims: &Claims) -> Result<(), AppError> {
        let existing = self
            .member_repository
            .find_by_workspace_and_email(workspace_identifier, &claims.email)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if let Some(member) = existing {
            if member.role != ROLE_OWNER {
                // An owner must exist for every workspace; promote only when
                // no owner row is present (legacy workspaces synced before
                // membership existed).
                let members = self
                    .member_repository
                    .list_by_workspace(workspace_identifier)
                    .await
                    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

                if !members.iter().any(|m| m.role == ROLE_OWNER) {
                    self.member_repository
                        .set_role(workspace_identifier, member.identifier, ROLE_OWNER)
                        .await
                        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
                }
            }
            return Ok(());
        }

        let members = self
            .member_repository
            .list_by_workspace(workspace_identifier)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !members.is_empty() {
            return Err(AppError::Forbidden(
                "You don't have access to this workspace".into(),
            ));
        }

        self.member_repository
            .insert(
                workspace_identifier,
                &claims.email,
                ROLE_OWNER,
                Some(claims.user_identifier),
            )
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Reject the request unless the account is a member of the workspace.
    pub async fn assert_member(&self, workspace_identifier: Uuid, email: &str) -> Result<(), AppError> {
        let member = self
            .member_repository
            .find_by_workspace_and_email(workspace_identifier, email)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if member.is_none() {
            return Err(AppError::Forbidden(
                "You don't have access to this workspace".into(),
            ));
        }

        Ok(())
    }

    /// Allow modification when the caller is a member, or when the workspace
    /// has no members yet (legacy workspaces synced before membership existed).
    pub async fn assert_can_modify(
        &self,
        workspace_identifier: Uuid,
        email: &str,
    ) -> Result<(), AppError> {
        let members = self
            .member_repository
            .list_by_workspace(workspace_identifier)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if members.is_empty() {
            return Ok(());
        }

        if !members
            .iter()
            .any(|m| m.member_email == email.to_lowercase())
        {
            return Err(AppError::Forbidden(
                "You don't have access to this workspace".into(),
            ));
        }

        Ok(())
    }

    pub async fn has_any_role(
        &self,
        workspace_identifier: Uuid,
        email: &str,
        roles: &[&str],
    ) -> Result<bool, AppError> {
        let member = self
            .member_repository
            .find_by_workspace_and_email(workspace_identifier, email)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(matches!(member, Some(m) if roles.contains(&m.role.as_str())))
    }

    pub async fn add_member(
        &self,
        workspace_identifier: Uuid,
        email: &str,
        user_identifier: Option<Uuid>,
    ) -> Result<workspace_members::Model, AppError> {
        if let Some(existing) = self
            .member_repository
            .find_by_workspace_and_email(workspace_identifier, email)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
        {
            return Ok(existing);
        }

        self.member_repository
            .insert(workspace_identifier, email, ROLE_MEMBER, user_identifier)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    pub async fn list_members(
        &self,
        workspace_identifier: Uuid,
    ) -> Result<Vec<workspace_members::Model>, AppError> {
        self.member_repository
            .list_by_workspace(workspace_identifier)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    /// List the roster of a workspace; only members may view it.
    pub async fn list_members_for_account(
        &self,
        workspace_identifier: Uuid,
        requester_email: &str,
    ) -> Result<Vec<workspace_members::Model>, AppError> {
        self.assert_member(workspace_identifier, requester_email)
            .await?;
        self.list_members(workspace_identifier).await
    }

    /// All workspaces the given account belongs to, with the account's role
    /// in each. This is the authoritative "one account, many workspaces"
    /// listing.
    pub async fn list_workspaces_for_account(
        &self,
        email: &str,
    ) -> Result<Vec<AccountWorkspaceResponse>, AppError> {
        let rows = self
            .member_repository
            .list_workspaces_for_email(email)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(rows
            .into_iter()
            .filter_map(|(member, workspace)| {
                workspace.map(|w| AccountWorkspaceResponse {
                    identifier: w.identifier.to_string(),
                    name: w.name,
                    description: w.description,
                    role: member.role,
                })
            })
            .collect())
    }

    /// Remove a non-owner member from a workspace. Only owners/admins may
    /// remove members.
    pub async fn remove_member(
        &self,
        workspace_identifier: Uuid,
        member_identifier: Uuid,
        requester_email: &str,
    ) -> Result<(), AppError> {
        let is_privileged = self
            .has_any_role(workspace_identifier, requester_email, &[ROLE_OWNER, ROLE_ADMIN])
            .await?;

        if !is_privileged {
            return Err(AppError::Forbidden(
                "Only workspace owners and admins can remove members".into(),
            ));
        }

        let target = self
            .member_repository
            .find_by_workspace_and_identifier(workspace_identifier, member_identifier)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Member not found".into()))?;

        if target.role == ROLE_OWNER {
            return Err(AppError::Forbidden(
                "Workspace owners cannot be removed".into(),
            ));
        }

        self.member_repository
            .delete(member_identifier)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
