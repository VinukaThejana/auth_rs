use crate::{auth_proto::RegisterRequest, util::verify};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct CreateUserReq {
    #[validate(email(message = "not valid"))]
    pub email: String,

    #[validate(custom(function = "verify::username"))]
    pub username: String,

    #[validate(length(
        min = 3,
        max = 100,
        message = "name must be between 3 and 100 characters"
    ))]
    pub name: String,

    #[validate(custom(function = "verify::password"))]
    pub password: String,
}

impl From<RegisterRequest> for CreateUserReq {
    fn from(value: RegisterRequest) -> Self {
        Self {
            email: value.email,
            username: value.username,
            name: value.name,
            password: value.password,
        }
    }
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct UserDetails {
    #[validate(length(min = 26, max = 26, message = "id must be 26 characters"))]
    id: String,

    #[validate(email(message = "not valid"))]
    pub email: String,

    #[validate(custom(function = "verify::username"))]
    pub username: String,

    #[validate(length(
        min = 3,
        max = 100,
        message = "name must be between 3 and 100 characters"
    ))]
    pub name: String,

    #[validate(url(message = "not valid"))]
    pub photo_url: String,

    pub is_two_factor_enabled: bool,
    pub is_email_verified: bool,
}

impl From<&UserDetails> for UserDetails {
    fn from(value: &UserDetails) -> Self {
        Self {
            id: value.id.clone(),
            email: value.email.clone(),
            username: value.username.clone(),
            name: value.name.clone(),
            photo_url: value.photo_url.clone(),
            is_email_verified: value.is_email_verified,
            is_two_factor_enabled: value.is_two_factor_enabled,
        }
    }
}

impl From<entity::user::Model> for UserDetails {
    fn from(value: entity::user::Model) -> Self {
        Self {
            id: value.id,
            email: value.email,
            username: value.username,
            name: value.name.clone(),
            photo_url: match value.photo_url {
                Some(url) => url,
                None => format!(
                    "https://api.dicebear.com/9.x/pixel-art/svg?seed={}",
                    value.name
                ),
            },
            is_two_factor_enabled: value.is_two_factor_enabled,
            is_email_verified: value.is_email_verified,
        }
    }
}

impl From<&entity::user::Model> for UserDetails {
    fn from(value: &entity::user::Model) -> Self {
        Self {
            id: value.id.to_owned(),
            email: value.email.to_owned(),
            username: value.username.to_owned(),
            name: value.name.to_owned(),
            photo_url: match value.photo_url.to_owned() {
                Some(url) => url,
                None => format!(
                    "https://api.dicebear.com/9.x/pixel-art/svg?seed={}",
                    value.name
                ),
            },
            is_two_factor_enabled: value.is_two_factor_enabled,
            is_email_verified: value.is_email_verified,
        }
    }
}
