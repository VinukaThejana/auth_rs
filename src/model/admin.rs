use crate::{
    admin_proto::{CreateAdminRequest, DeleteAdminRequest},
    util::verify,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct CreateAdminReq {
    #[validate(email(message = "not valid"))]
    pub email: String,

    #[validate(custom(function = "verify::otp"))]
    pub otp: String,

    #[validate(length(
        min = 5,
        max = 150,
        message = "description must be between 5 and 150 characters"
    ))]
    pub description: String,
}

impl From<CreateAdminRequest> for CreateAdminReq {
    fn from(value: CreateAdminRequest) -> Self {
        Self {
            email: value.email,
            otp: value.otp,
            description: value.description,
        }
    }
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct DeleteAdminReq {
    #[validate(email(message = "not valid"))]
    pub email: String,

    #[validate(custom(function = "verify::otp"))]
    pub otp: String,
}

impl From<DeleteAdminRequest> for DeleteAdminReq {
    fn from(value: DeleteAdminRequest) -> Self {
        Self {
            email: value.email,
            otp: value.otp,
        }
    }
}
