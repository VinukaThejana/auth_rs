use crate::{
    admin_proto::{
        CreateApiKeyRequest, DeleteApiKeyRequest, ListApiKeysRequest, ListApiKeysResponse,
        list_api_keys_response::ApiKey,
    },
    util::verify,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIKey {
    pub key: String,
    pub secret: String,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct ListApiKeysReq {
    #[validate(email(message = "not valid"))]
    pub email: String,

    #[validate(custom(function = "verify::otp"))]
    pub otp: String,
}

impl From<ListApiKeysRequest> for ListApiKeysReq {
    fn from(value: ListApiKeysRequest) -> Self {
        Self {
            email: value.email,
            otp: value.otp,
        }
    }
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct CreateApiKeyReq {
    #[validate(email(message = "not valid"))]
    pub email: String,

    #[validate(custom(function = "verify::otp"))]
    pub otp: String,

    #[validate(length(min = 10, max = 150, message = "must be between 10 and 150 characters"))]
    pub description: String,
}

impl From<CreateApiKeyRequest> for CreateApiKeyReq {
    fn from(value: CreateApiKeyRequest) -> Self {
        Self {
            email: value.email,
            otp: value.otp,
            description: value.description,
        }
    }
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct DeleteApiKeyReq {
    #[validate(email(message = "not valid"))]
    pub email: String,

    #[validate(custom(function = "verify::otp"))]
    pub otp: String,

    #[validate(length(min = 26, max = 26, message = "must be a valid key"))]
    pub key: String,
}

impl From<DeleteApiKeyRequest> for DeleteApiKeyReq {
    fn from(value: DeleteApiKeyRequest) -> Self {
        Self {
            email: value.email,
            otp: value.otp,
            key: value.api_key,
        }
    }
}

impl From<entity::admin_api_key::Model> for ApiKey {
    fn from(value: entity::admin_api_key::Model) -> Self {
        Self {
            api_key: value.key,
            description: value.description,
            created_at: value.created_at.to_string(),
        }
    }
}

impl From<Vec<entity::admin_api_key::Model>> for ListApiKeysResponse {
    fn from(value: Vec<entity::admin_api_key::Model>) -> Self {
        Self {
            api_keys: value.into_iter().map(ApiKey::from).collect(),
        }
    }
}
