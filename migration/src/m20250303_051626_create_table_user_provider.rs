use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250302_192622_create_table_user::User, m20250303_051455_create_table_provider::Provider,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum UserProvider {
    Table,
    UserId,
    ProviderId,
    ProivderGivenUserId,
    LinkedAt,
}

const IDX_USER_ID: &str = "idx_user_provider_user_id";
const IDX_PROVIDER_ID: &str = "idx_user_provider_provider_id";

const FK_USER_ID: &str = "fk_user_provider_user_id";
const FK_PROVIDER_ID: &str = "fk_user_provider_provider_id";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserProvider::Table)
                    .if_not_exists()
                    .col(string(UserProvider::UserId).char().char_len(26))
                    .col(string(UserProvider::ProviderId).string_len(150))
                    .col(string_null(UserProvider::ProivderGivenUserId).string_len(255))
                    .col(date_time(UserProvider::LinkedAt).extra("DEFAULT CURRENT_TIMESTAMP"))
                    .primary_key(
                        Index::create()
                            .col(UserProvider::UserId)
                            .col(UserProvider::ProviderId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(UserProvider::Table)
                    .col(UserProvider::UserId)
                    .name(IDX_USER_ID)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(UserProvider::Table)
                    .col(UserProvider::ProviderId)
                    .name(IDX_PROVIDER_ID)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name(FK_USER_ID)
                    .from(UserProvider::Table, UserProvider::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Restrict)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name(FK_PROVIDER_ID)
                    .from(UserProvider::Table, UserProvider::ProviderId)
                    .to(Provider::Table, Provider::Id)
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
                    .table(UserProvider::Table)
                    .name(IDX_USER_ID)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .table(UserProvider::Table)
                    .name(IDX_PROVIDER_ID)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_USER_ID)
                    .table(UserProvider::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_PROVIDER_ID)
                    .table(UserProvider::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(UserProvider::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
