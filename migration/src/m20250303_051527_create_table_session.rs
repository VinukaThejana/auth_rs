use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250302_192622_create_table_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Session {
    Table,
    Id,
    UserId,
    IpAddress,
    LoginAt,
    Exp,
    DeviceVendor,
    DeviceModel,
    OsName,
    OsVersion,
    BrowserName,
    BorwserVersion,
    Country,
    City,
    Region,
    Timezone,
    Lat,
    Lon,
    MapURL,
}

const IDX_USER_ID: &str = "idx_session_user_id";
const IDX_IP_ADDRESS: &str = "idx_session_ip_address";
const IDX_LOGIN_AT: &str = "idx_session_login_at";
const IDX_EXP: &str = "idx_session_exp";

const FK_USER_ID: &str = "fk_session_user_id";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(string(Session::Id).char().char_len(26).primary_key())
                    .col(string(Session::UserId).char().char_len(26))
                    .col(string(Session::IpAddress).string_len(45))
                    .col(integer(Session::LoginAt).unsigned())
                    .col(integer(Session::Exp).unsigned())
                    .col(string_null(Session::DeviceVendor).string_len(255))
                    .col(string_null(Session::DeviceModel).string_len(255))
                    .col(string_null(Session::OsName).string_len(255))
                    .col(string_null(Session::OsVersion).string_len(255))
                    .col(string_null(Session::BrowserName).string_len(255))
                    .col(string_null(Session::BorwserVersion).string_len(255))
                    .col(string_null(Session::Country).string_len(80))
                    .col(string_null(Session::City).string_len(100))
                    .col(string_null(Session::Region).string_len(80))
                    .col(string_null(Session::Timezone).string_len(50))
                    .col(float_null(Session::Lat).decimal_len(10, 8))
                    .col(float_null(Session::Lon).decimal_len(11, 8))
                    .col(string_null(Session::MapURL).string_len(255))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Session::Table)
                    .col(Session::UserId)
                    .name(IDX_USER_ID)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Session::Table)
                    .col(Session::LoginAt)
                    .name(IDX_LOGIN_AT)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Session::Table)
                    .col(Session::Exp)
                    .name(IDX_EXP)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(Session::Table)
                    .col(Session::IpAddress)
                    .name(IDX_IP_ADDRESS)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name(FK_USER_ID)
                    .from(Session::Table, Session::UserId)
                    .to(User::Table, User::Id)
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
                    .name(IDX_USER_ID)
                    .table(Session::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name(IDX_IP_ADDRESS)
                    .table(Session::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name(IDX_LOGIN_AT)
                    .table(Session::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name(IDX_EXP)
                    .table(Session::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name(FK_USER_ID)
                    .table(Session::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Session::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}
