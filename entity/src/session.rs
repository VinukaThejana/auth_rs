//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(schema_name = "auth", table_name = "session")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub user_id: String,
    pub ip_address: String,
    pub login_at: i32,
    pub exp: i32,
    pub device_vendor: Option<String>,
    pub device_model: Option<String>,
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub browser_name: Option<String>,
    pub borwser_version: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub timezone: Option<String>,
    #[sea_orm(column_type = "Decimal(Some((10, 8)))", nullable)]
    pub lat: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((11, 8)))", nullable)]
    pub lon: Option<Decimal>,
    pub map_url: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Restrict",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
