use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Admin {
    Table,
    Id,
    Email,
    Description,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Admin::Table)
                    .if_not_exists()
                    .col(
                        string(Admin::Id)
                            .char()
                            .char_len(26)
                            .primary_key()
                            .extra("DEFAULT public.gen_ulid()"),
                    )
                    .col(string(Admin::Email).string_len(255).unique_key())
                    .col(text(Admin::Description))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Admin::Table).to_owned())
            .await
    }
}
