use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250314_124122_admin::Admin;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum AdminApiKey {
    Table,
    Id,
    Key,
    Description,
    OwnedBy,
    CreatedAt,
    LastUsed,
}

const IDX_USER_ID: &str = "idx_admin_api_key_owned_by";
const FK_OWNED_BY: &str = "fk_admin_api_key_owned_by";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AdminApiKey::Table)
                    .if_not_exists()
                    .col(
                        string(AdminApiKey::Id)
                            .char()
                            .char_len(26)
                            .primary_key()
                            .extra("DEFAULT public.gen_ulid()"),
                    )
                    .col(string(AdminApiKey::Key).string_len(255))
                    .col(text(AdminApiKey::Description))
                    .col(string(AdminApiKey::OwnedBy).string_len(255))
                    .col(date_time(AdminApiKey::CreatedAt).extra("DEFAULT CURRENT_TIMESTAMP"))
                    .col(date_time(AdminApiKey::LastUsed).extra("DEFAULT CURRENT_TIMESTAMP"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(AdminApiKey::Table)
                    .col(AdminApiKey::OwnedBy)
                    .name(IDX_USER_ID)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name(FK_OWNED_BY)
                    .from(AdminApiKey::Table, AdminApiKey::OwnedBy)
                    .to(Admin::Table, Admin::Email)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .table(AdminApiKey::Table)
                    .name(IDX_USER_ID)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_OWNED_BY)
                    .table(AdminApiKey::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(AdminApiKey::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
