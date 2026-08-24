use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260224_214545_create_workspaces::Workspaces;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(WorkspaceMembers::Table)
            .if_not_exists()
            .col(pk_uuid(WorkspaceMembers::Identifier))
            .col(string(WorkspaceMembers::MemberEmail))
            .col(string(WorkspaceMembers::Role))
            .col(uuid_null(WorkspaceMembers::UserIdentifier))
            .col(timestamp_with_time_zone(WorkspaceMembers::CreatedAt))
            .col(timestamp_with_time_zone(WorkspaceMembers::UpdatedAt))
            .col(uuid(WorkspaceMembers::WorkspaceIdentifier))
            .foreign_key(
                ForeignKey::create()
                    .name("fk_workspace_members_workspace_identifier")
                    .from(
                        WorkspaceMembers::Table,
                        WorkspaceMembers::WorkspaceIdentifier,
                    )
                    .to(Workspaces::Table, Workspaces::Identifier)
                    .on_delete(ForeignKeyAction::Cascade),
            )
            .to_owned();

        manager.create_table(table).await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_workspace_members_workspace_email")
                    .table(WorkspaceMembers::Table)
                    .col(WorkspaceMembers::WorkspaceIdentifier)
                    .col(WorkspaceMembers::MemberEmail)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorkspaceMembers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum WorkspaceMembers {
    Table,
    Identifier,
    WorkspaceIdentifier,
    MemberEmail,
    UserIdentifier,
    Role,
    CreatedAt,
    UpdatedAt,
}
