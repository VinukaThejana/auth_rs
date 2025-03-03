use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Email,
    Username,
    Name,
    Password,
    PhotoURL,
    IsEmailVerified,
    IsTwoFactorEnabled,
}

const IDX_EMAIL: &str = "idx_user_email";
const IDX_USERNAME: &str = "idx_user_username";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        string(User::Id)
                            .char()
                            .char_len(26)
                            .primary_key()
                            .extra("DEFAULT public.gen_ulid()"),
                    )
                    .col(string(User::Email).string_len(255).unique_key())
                    .col(string(User::Username).string_len(255).unique_key())
                    .col(string(User::Name).string_len(255))
                    .col(string_null(User::Password).string_len(255))
                    .col(string_null(User::PhotoURL).string_len(255))
                    .col(boolean(User::IsEmailVerified).extra("DEFAULT FALSE"))
                    .col(boolean(User::IsTwoFactorEnabled).extra("DEFAULT FALSE"))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(IDX_EMAIL)
                    .table(User::Table)
                    .col(User::Email)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name(IDX_USERNAME)
                    .table(User::Table)
                    .col(User::Username)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name(IDX_EMAIL)
                    .table(User::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name(IDX_USERNAME)
                    .table(User::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}
