use sea_orm::{
    ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set, TransactionTrait,
    entity::*,
};

pub async fn create(
    db: &DatabaseConnection,
    provider: &str,
    email: &str,
    username: &str,
    name: &str,
    password: Option<&str>,
    photo_url: Option<&str>,
) -> Result<entity::user::Model, DbErr> {
    let txn = db.begin().await?;

    let user = entity::user::ActiveModel {
        email: Set(email.to_owned()),
        username: Set(username.to_owned()),
        name: Set(name.to_owned()),
        photo_url: Set(match photo_url {
            Some(photo_url) => Some(photo_url.to_owned()),
            None => Some(format!(
                "https://api.dicebear.com/9.x/pixel-art/svg?seed={}",
                name
            )),
        }),
        password: Set(match password {
            Some(password) => Some(
                bcrypt::hash(password, bcrypt::DEFAULT_COST)
                    .map_err(|err| DbErr::Custom(err.to_string()))?,
            ),
            None => None,
        }),
        is_two_factor_enabled: Set(true),
        is_email_verified: Set(false),
        ..Default::default()
    };
    let user = user.insert(&txn).await?;

    let user_provider = entity::user_provider::ActiveModel {
        user_id: Set(user.id.clone()),
        provider_id: Set(provider.to_owned()),
        ..Default::default()
    };
    let _ = user_provider.insert(&txn).await?;

    txn.commit().await?;
    Ok(user)
}

pub async fn get_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<entity::user::Model, DbErr> {
    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(email))
        .one(db)
        .await?;
    let user = user.ok_or(DbErr::RecordNotFound(String::from(
        "user with the given email does not exist",
    )))?;

    Ok(user)
}

pub async fn get_by_credential(
    db: &DatabaseConnection,
    credential: &str,
) -> Result<entity::user::Model, DbErr> {
    let user = entity::user::Entity::find()
        .filter(
            entity::user::Column::Email
                .eq(credential)
                .or(entity::user::Column::Username.eq(credential)),
        )
        .one(db)
        .await?;
    let user = user.ok_or(DbErr::RecordNotFound(String::from(
        "user with the given credential does not exist",
    )))?;

    Ok(user)
}
