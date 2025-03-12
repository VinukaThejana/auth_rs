use crate::config::ENV;
use std::fmt::{Display, Formatter, Result};

pub mod claims;
pub mod error;
pub mod params;
pub mod response;
pub mod service;
pub mod traits;
pub mod types;

pub enum TokenType {
    Access,
    Refresh,
    Session,
    ReAuth,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let token: &str = match *self {
            Self::Access => "access_token",
            Self::Refresh => "refresh_token",
            Self::Session => "session_token",
            Self::ReAuth => "reauth_token",
        };

        write!(f, "{}", token)
    }
}

impl TokenType {
    pub fn get_key(&self, jti: &str) -> String {
        format!("{}:{}:{}", &*ENV.redis_schema, self, jti)
    }
}
