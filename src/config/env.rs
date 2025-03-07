use crate::{
    error::AppError,
    util::{deserialize_arc_str, deserialize_base64},
};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::process::exit;
use std::sync::Arc;
use time::Duration;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Env {
    #[validate(length(min = 1, message = "DATABASE_URL is required"))]
    #[serde(deserialize_with = "deserialize_arc_str")]
    pub database_url: Arc<str>,

    #[validate(length(min = 1, message = "DATABASE_SCHEMA is required"))]
    #[serde(deserialize_with = "deserialize_arc_str")]
    pub database_schema: Arc<str>,

    #[validate(length(min = 1, message = "REDIS_URL is required"))]
    #[serde(deserialize_with = "deserialize_arc_str")]
    pub redis_url: Arc<str>,

    #[validate(length(min = 1, message = "REDIS_SCHEMA is required"))]
    #[serde(deserialize_with = "deserialize_arc_str")]
    pub redis_schema: Arc<str>,

    #[validate(custom(function = "envmode::verify"))]
    #[serde(deserialize_with = "deserialize_arc_str")]
    pub env: Arc<str>,

    #[validate(length(min = 1, message = "DOMAIN is required"))]
    #[serde(deserialize_with = "deserialize_arc_str")]
    pub domain: Arc<str>,

    #[validate(length(min = 1, message = "IPINFO_API_KEY is required"))]
    #[serde(deserialize_with = "deserialize_arc_str")]
    pub ipinfo_api_key: Arc<str>,

    #[validate(length(min = 1, message = "REFRESH_TOKEN_PRIVATE_KEY is required"))]
    #[serde(deserialize_with = "deserialize_base64")]
    pub refresh_token_private_key: Arc<Vec<u8>>,

    #[validate(length(min = 1, message = "REFRESH_TOKEN_PUBLIC_KEY is required"))]
    #[serde(deserialize_with = "deserialize_base64")]
    pub refresh_token_public_key: Arc<Vec<u8>>,

    #[validate(length(min = 1, message = "ACCESS_TOKEN_PRIVATE_KEY is required"))]
    #[serde(deserialize_with = "deserialize_base64")]
    pub access_token_private_key: Arc<Vec<u8>>,

    #[validate(length(min = 1, message = "ACCESS_TOKEN_PUBLIC_KEY is required"))]
    #[serde(deserialize_with = "deserialize_base64")]
    pub access_token_public_key: Arc<Vec<u8>>,

    #[validate(length(min = 1, message = "SESSION_TOKEN_PRIVATE_KEY is required"))]
    #[serde(deserialize_with = "deserialize_base64")]
    pub session_token_private_key: Arc<Vec<u8>>,

    #[validate(length(min = 1, message = "SESSION_TOKEN_PUBLIC_KEY is required"))]
    #[serde(deserialize_with = "deserialize_base64")]
    pub session_token_public_key: Arc<Vec<u8>>,

    #[validate(length(min = 1, message = "REAUTH_TOKEN_PRIVATE_KEY is required"))]
    #[serde(deserialize_with = "deserialize_base64")]
    pub reauth_token_private_key: Arc<Vec<u8>>,

    #[validate(length(min = 1, message = "REAUTH_TOKEN_PUBLIC_KEY is required"))]
    #[serde(deserialize_with = "deserialize_base64")]
    pub reauth_token_public_key: Arc<Vec<u8>>,

    #[validate(range(
        min = TryInto::<usize>::try_into(Duration::days(15).whole_seconds()).unwrap(),
        max = TryInto::<usize>::try_into(Duration::days(90).whole_seconds()).unwrap(),
        message = "REFRESH_TOKEN_EXPIRATION must be between 15 days and 3 months"
    ))]
    pub refresh_token_expiration: usize,

    #[validate(range(
        min = TryInto::<usize>::try_into(Duration::days(15).whole_seconds()).unwrap(),
        max = TryInto::<usize>::try_into(Duration::days(90).whole_seconds()).unwrap(),
        message = "SESSION_TOKEN_EXPIRATION must be between 15 days and 3 months"
    ))]
    pub session_token_expiration: usize,

    #[validate(range(
        min = TryInto::<usize>::try_into(Duration::minutes(30).whole_seconds()).unwrap(),
        max = TryInto::<usize>::try_into(Duration::hours(6).whole_seconds()).unwrap(),
        message = "ACCESS_TOKEN_EXPIRATION must be between 30 minutes and 6 hours"
    ))]
    pub access_token_expiration: usize,

    #[validate(range(
        min = TryInto::<usize>::try_into(Duration::minutes(1).whole_seconds()).unwrap(),
        max = TryInto::<usize>::try_into(Duration::minutes(10).whole_seconds()).unwrap(),
        message = "REAUTH_TOKEN_EXPIRATION must be between 1 minute and 10 minutes"
    ))]
    pub reauth_token_expiration: usize,

    #[validate(range(
        min = 50050,
        max = 50060,
        message = "PORT must be between 8080 and 8090"
    ))]
    pub port: u16,
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}

impl Env {
    pub fn new() -> Self {
        let _ = dotenv();

        let env: Self = envy::from_env().unwrap_or_else(|e| {
            eprintln!("{}, exiting ... ", e);
            exit(1);
        });
        env.validate().unwrap_or_else(|e| {
            eprintln!("{}", AppError::Validation(e));
            eprintln!("update the environment and try again, exiting ... ");
            exit(1);
        });

        env_logger::init();
        env
    }
}

pub static ENV: Lazy<Env> = Lazy::new(Env::new);
