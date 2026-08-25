use std::sync::Arc;

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use chrono::Local;
use crate::{
    adapters::invitation::InviteWorkspaceMemberRequest,

    errors::database_error::DatabaseError,
    repositories::base::Repository,
};
use lunar::entities::invitation::{self, ActiveModel, Entity as InvitationEntity};

#[derive(Clone)]
pub struct InvitationRepository {
    db_conn: Arc<DatabaseConnection>,
}

impl Repository for InvitationRepository {
    fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            db_conn: db_conn.to_owned(),
        }
    }
}

pub(crate) trait InvitationRepositoryTrait {
    async fn find_by_email_and_workspace(
        &self,
        workspace_identifier: Uuid,
        email: &str,
    ) -> Result<Option<invitation::Model>, DatabaseError>;

    async fn find_by_token(&self, token: &str) -> Result<Option<invitation::Model>, DatabaseError>;

    async fn find_by_identifier(
        &self,
        identifier: Uuid,
    ) -> Result<Option<invitation::Model>, DatabaseError>;

    async fn create(
        &self,
        workspace_identifier: Uuid,
        req: &InviteWorkspaceMemberRequest,
        token: &str,
    ) -> Result<invitation::Model, DatabaseError>;

    async fn update_status(
        &self,
        invitation: invitation::Model,
        status: &str,
    ) -> Result<invitation::Model, DatabaseError>;

    async fn delete(&self, identifier: Uuid) -> Result<bool, DatabaseError>;
}

impl InvitationRepositoryTrait for InvitationRepository {
    async fn find_by_email_and_workspace(
        &self,
        workspace_identifier: Uuid,
        email: &str,
    ) -> Result<Option<invitation::Model>, DatabaseError> {
        InvitationEntity::find()
            .filter(invitation::Column::WorkspaceIdentifier.eq(workspace_identifier))
            .filter(invitation::Column::Email.eq(email))
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)
    }

    async fn find_by_token(&self, token: &str) -> Result<Option<invitation::Model>, DatabaseError> {
        InvitationEntity::find()
            .filter(invitation::Column::Token.eq(token))
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)
    }

    async fn find_by_identifier(
        &self,
        identifier: Uuid,
    ) -> Result<Option<invitation::Model>, DatabaseError> {
        InvitationEntity::find_by_id(identifier)
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)
    }

    async fn update_status(
        &self,
        invitation: invitation::Model,
        status: &str,
    ) -> Result<invitation::Model, DatabaseError> {
        let mut active: invitation::ActiveModel = invitation.into();
        active.status = Set(status.to_owned());
        active
            .update(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)
    }

    async fn delete(&self, identifier: Uuid) -> Result<bool, DatabaseError> {
        let result = InvitationEntity::delete_by_id(identifier)
            .exec(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;
        Ok(result.rows_affected > 0)
    }

    async fn create(
        &self,
        workspace_identifier: Uuid,
        req: &InviteWorkspaceMemberRequest,
        token: &str,
    ) -> Result<invitation::Model, DatabaseError> {
        let model = ActiveModel {
            identifier: Set(Uuid::new_v4()),
            workspace_identifier: Set(workspace_identifier),
            email: Set(req.email.clone()),
            first_name: Set(req.first_name.clone()),
            last_name: Set(req.last_name.clone()),
            token: Set(token.to_owned()),
            status: Set("pending".to_string()),
            expires_at: Set((Local::now() + chrono::Duration::days(7)).into()),
            created_at: Set(Local::now().into()),
        };
        model
            .insert(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)
    }
}
