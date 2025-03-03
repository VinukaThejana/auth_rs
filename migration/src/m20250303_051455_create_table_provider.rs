use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Provider {
    Table,
    Id,
    Name,
    LogoURL,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Provider::Table)
                    .if_not_exists()
                    .col(string(Provider::Id).string_len(150).primary_key())
                    .col(string(Provider::Name).string_len(150).not_null())
                    .col(string(Provider::LogoURL).string_len(150).not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Provider::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}
