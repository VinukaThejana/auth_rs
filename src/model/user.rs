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
