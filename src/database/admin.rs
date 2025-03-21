use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set, entity::*,
};

pub async fn create(
    db: &DatabaseConnection,
    email: &str,
    description: &str,
) -> Result<entity::admin::Model, DbErr> {
    let admin = entity::admin::ActiveModel {
        email: Set(email.to_owned()),
        description: Set(description.to_owned()),
        ..Default::default()
    };
    let admin = admin.insert(db).await?;

    Ok(admin)
}

pub async fn get_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<entity::admin::Model, DbErr> {
    let admin = entity::admin::Entity::find()
        .filter(entity::admin::Column::Email.eq(email.to_lowercase()))
        .one(db)
        .await?;
    let admin = admin.ok_or(DbErr::RecordNotFound(String::from(
        "admin with the given email does not exist",
    )))?;

    Ok(admin)
}

pub async fn delete(db: &DatabaseConnection, email: &str) -> Result<(), DbErr> {
    let admin = entity::admin::Entity::find()
        .filter(entity::admin::Column::Email.eq(email.to_owned()))
        .one(db)
        .await?;
    let admin: entity::admin::Model = admin.ok_or(DbErr::RecordNotFound(String::from(
        "admin with the given email does not exist",
    )))?;
    admin.delete(db).await?;

    Ok(())
}
