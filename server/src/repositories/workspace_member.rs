use std::sync::Arc;

use chrono::Utc;
use lunar::entities::{workspaces, workspace_members};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::{errors::database_error::DatabaseError, repositories::base::Repository};

#[derive(Clone)]
pub struct WorkspaceMemberRepository {
    db_conn: Arc<DatabaseConnection>,
}

impl Repository for WorkspaceMemberRepository {
    fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            db_conn: db_conn.to_owned(),
        }
    }
}

pub(crate) trait WorkspaceMemberRepositoryTrait {
    async fn find_by_workspace_and_email(
        &self,
        workspace_identifier: Uuid,
        email: &str,
    ) -> Result<Option<workspace_members::Model>, DatabaseError>;

    async fn find_by_workspace_and_identifier(
        &self,
        workspace_identifier: Uuid,
        member_identifier: Uuid,
    ) -> Result<Option<workspace_members::Model>, DatabaseError>;

    async fn list_by_workspace(
        &self,
        workspace_identifier: Uuid,
    ) -> Result<Vec<workspace_members::Model>, DatabaseError>;

    async fn list_workspaces_for_email(
        &self,
        email: &str,
    ) -> Result<Vec<(workspace_members::Model, Option<workspaces::Model>)>, DatabaseError>;

    async fn insert(
        &self,
        workspace_identifier: Uuid,
        email: &str,
        role: &str,
        user_identifier: Option<Uuid>,
    ) -> Result<workspace_members::Model, DatabaseError>;

    async fn set_role(
        &self,
        workspace_identifier: Uuid,
        member_identifier: Uuid,
        role: &str,
    ) -> Result<workspace_members::Model, DatabaseError>;

    async fn delete(&self, identifier: Uuid) -> Result<bool, DatabaseError>;
}

impl WorkspaceMemberRepositoryTrait for WorkspaceMemberRepository {
    async fn find_by_workspace_and_email(
        &self,
        workspace_identifier: Uuid,
        email: &str,
    ) -> Result<Option<workspace_members::Model>, DatabaseError> {
        workspace_members::Entity::find()
            .filter(workspace_members::Column::WorkspaceIdentifier.eq(workspace_identifier))
            .filter(
                workspace_members::Column::MemberEmail.eq(email.to_lowercase()),
            )
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)
    }

    async fn find_by_workspace_and_identifier(
        &self,
        workspace_identifier: Uuid,
        member_identifier: Uuid,
    ) -> Result<Option<workspace_members::Model>, DatabaseError> {
        workspace_members::Entity::find()
            .filter(workspace_members::Column::WorkspaceIdentifier.eq(workspace_identifier))
            .filter(workspace_members::Column::Identifier.eq(member_identifier))
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)
    }

    async fn list_by_workspace(
        &self,
        workspace_identifier: Uuid,
    ) -> Result<Vec<workspace_members::Model>, DatabaseError> {
        workspace_members::Entity::find()
            .filter(workspace_members::Column::WorkspaceIdentifier.eq(workspace_identifier))
            .all(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)
    }

    async fn list_workspaces_for_email(
        &self,
        email: &str,
    ) -> Result<Vec<(workspace_members::Model, Option<workspaces::Model>)>, DatabaseError> {
        workspace_members::Entity::find()
            .filter(workspace_members::Column::MemberEmail.eq(email.to_lowercase()))
            .find_also_related(workspaces::Entity)
            .all(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)
    }

    async fn insert(
        &self,
        workspace_identifier: Uuid,
        email: &str,
        role: &str,
        user_identifier: Option<Uuid>,
    ) -> Result<workspace_members::Model, DatabaseError> {
        let now = Utc::now().fixed_offset();
        let model = workspace_members::ActiveModel {
            identifier: Set(Uuid::new_v4()),
            member_email: Set(email.to_lowercase()),
            role: Set(role.to_owned()),
            user_identifier: Set(user_identifier),
            created_at: Set(now),
            updated_at: Set(now),
            workspace_identifier: Set(workspace_identifier),
        };
        model
            .insert(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)
    }

    async fn set_role(
        &self,
        workspace_identifier: Uuid,
        member_identifier: Uuid,
        role: &str,
    ) -> Result<workspace_members::Model, DatabaseError> {
        let existing = workspace_members::Entity::find()
            .filter(workspace_members::Column::WorkspaceIdentifier.eq(workspace_identifier))
            .filter(workspace_members::Column::Identifier.eq(member_identifier))
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?
            .ok_or(DatabaseError::RecordNotFound)?;

        let mut active: workspace_members::ActiveModel = existing.into();
        active.role = Set(role.to_owned());
        active.updated_at = Set(Utc::now().fixed_offset());
        active
            .update(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)
    }

    async fn delete(&self, identifier: Uuid) -> Result<bool, DatabaseError> {
        let result = workspace_members::Entity::delete_by_id(identifier)
            .exec(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;
        Ok(result.rows_affected > 0)
    }
}
