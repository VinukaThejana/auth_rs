use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set, entity::*,
};

use crate::model::api::APIKey;

pub async fn list(
    db: &DatabaseConnection,
    id: &str,
) -> Result<Vec<entity::admin_api_key::Model>, DbErr> {
    let api_keys: Vec<entity::admin_api_key::Model> = entity::admin_api_key::Entity::find()
        .filter(entity::admin_api_key::Column::OwnedBy.eq(id))
        .all(db)
        .await?;

    Ok(api_keys)
}

pub async fn create(db: &DatabaseConnection, id: &str, description: &str) -> Result<APIKey, DbErr> {
    let api_key = format!("au_{}", ulid::Ulid::new().to_string());

    let admin_api_key = entity::admin_api_key::ActiveModel {
        key: Set(bcrypt::hash(&api_key, bcrypt::DEFAULT_COST)
            .map_err(|err| DbErr::Custom(err.to_string()))?),
        owned_by: Set(id.to_owned()),
        description: Set(description.to_owned()),
        ..Default::default()
    };
    let admin_api_key = admin_api_key.save(db).await?;
    let key = admin_api_key.id.unwrap();

    Ok(APIKey {
        key,
        secret: api_key,
    })
}

pub async fn delete(db: &DatabaseConnection, id: &str) -> Result<(), DbErr> {
    let _ = entity::admin_api_key::Entity::delete_by_id(id)
        .exec(db)
        .await?;

    Ok(())
}
