use std::sync::Arc;

use chrono::Utc;
use lunar::{
    entities::workspaces,
    repositories::workspace::{WorkspaceRepository, WorkspaceRepositoryExt},
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;
use chrono::Local;
use crate::{
    adapters::{
        invitation::{InviteWorkspaceMemberRequest, InviteWorkspaceMemberResponse},
        jwt::Claims,
    },
    errors::app_error::AppError,
    repositories::{
        base::Repository,
        invitation::{InvitationRepository, InvitationRepositoryTrait},
    },
    services::workspace_member_service::{
        WorkspaceMemberService, ROLE_ADMIN, ROLE_OWNER,
    },
};

#[derive(Clone)]
pub struct InvitationService {
    db_conn: Arc<DatabaseConnection>,
    invitation_repository: InvitationRepository,
    member_service: WorkspaceMemberService,
}

impl InvitationService {
    pub fn new(
        db_conn: Arc<DatabaseConnection>,
        invitation_repository: InvitationRepository,
        member_service: WorkspaceMemberService,
    ) -> Self {
        Self {
            db_conn,
            invitation_repository,
            member_service,
        }
    }

    pub fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            db_conn: db_conn.to_owned(),
            invitation_repository: InvitationRepository::init(db_conn),
            member_service: WorkspaceMemberService::init(db_conn),
        }
    }

    /// Invite a member to a workspace. Only owners/admins of an existing
    /// workspace may send invitations.
    pub async fn invite_member(
        &self,
        workspace_identifier: Uuid,
        claims: &Claims,
        payload: &InviteWorkspaceMemberRequest,
    ) -> Result<InviteWorkspaceMemberResponse, AppError> {
        self.load_workspace(workspace_identifier).await?;

        let is_privileged = self
            .member_service
            .has_any_role(
                workspace_identifier,
                &claims.email,
                &[ROLE_OWNER, ROLE_ADMIN],
            )
            .await?;

        if !is_privileged {
            return Err(AppError::Forbidden(
                "Only workspace owners and admins can invite members".into(),
            ));
        }

        if let Some(existing) = self
            .invitation_repository
            .find_by_email_and_workspace(workspace_identifier, &payload.email)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
        {
            if existing.status == "pending" {
                return Err(AppError::OperationFailed(
                    "An invitation is already pending for this email".into(),
                ));
            }
        }

        let token = Uuid::new_v4().to_string();

        let invitation = self
            .invitation_repository
            .create(workspace_identifier, payload, &token)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(InviteWorkspaceMemberResponse {
            invitation_id: invitation.identifier.to_string(),
            email: invitation.email,
            status: invitation.status,
        })
    }

    /// Accept a pending invitation as the authenticated account and add the
    /// account as a workspace member.
    pub async fn accept(&self, token: &str, claims: &Claims) -> Result<workspaces::Model, AppError> {
        let invitation = self
            .invitation_repository
            .find_by_token(token)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Invitation not found".into()))?;

        if invitation.status != "pending" {
            return Err(AppError::OperationFailed(format!(
                "This invitation has already been {}",
                invitation.status
            )));
        }

        if invitation.expires_at < Local::now() {
            self.invitation_repository
                .update_status(invitation.clone(), "expired")
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            return Err(AppError::OperationFailed("This invitation has expired".into()));
        }

        if !invitation.email.eq_ignore_ascii_case(&claims.email) {
            return Err(AppError::Forbidden(
                "This invitation was sent to another email address".into(),
            ));
        }

        self.invitation_repository
            .update_status(invitation.clone(), "accepted")
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        self.member_service
            .add_member(
                invitation.workspace_identifier,
                &claims.email,
                Some(claims.user_identifier),
            )
            .await?;

        self.load_workspace(invitation.workspace_identifier).await
    }

    /// Revoke a pending invitation. Only owners/admins of the target
    /// workspace may revoke.
    pub async fn revoke(&self, invitation_identifier: Uuid, claims: &Claims) -> Result<(), AppError> {
        let invitation = self
            .invitation_repository
            .find_by_identifier(invitation_identifier)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Invitation not found".into()))?;

        let is_privileged = self
            .member_service
            .has_any_role(
                invitation.workspace_identifier,
                &claims.email,
                &[ROLE_OWNER, ROLE_ADMIN],
            )
            .await?;

        if !is_privileged {
            return Err(AppError::Forbidden(
                "Only workspace owners and admins can revoke invitations".into(),
            ));
        }

        if invitation.status != "pending" {
            return Err(AppError::OperationFailed(
                "Only pending invitations can be revoked".into(),
            ));
        }

        self.invitation_repository
            .delete(invitation_identifier)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn load_workspace(&self, identifier: Uuid) -> Result<workspaces::Model, AppError> {
        WorkspaceRepository::new(self.db_conn.clone())
            .get_workspace_by_id(identifier)
            .await
            .map_err(AppError::LunarError)
    }
}
