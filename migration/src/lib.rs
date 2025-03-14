pub use sea_orm_migration::prelude::*;

mod m20250302_192622_create_table_user;
mod m20250303_051455_create_table_provider;
mod m20250303_051527_create_table_session;
mod m20250303_051626_create_table_user_provider;
mod m20250314_124122_admin;
mod m20250314_124135_admin_api_key;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250302_192622_create_table_user::Migration),
            Box::new(m20250303_051455_create_table_provider::Migration),
            Box::new(m20250303_051527_create_table_session::Migration),
            Box::new(m20250303_051626_create_table_user_provider::Migration),
            Box::new(m20250314_124122_admin::Migration),
            Box::new(m20250314_124135_admin_api_key::Migration),
        ]
    }
}
