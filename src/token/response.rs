use super::claims::Claims;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub struct Factory<T>
where
    T: Claims,
{
    pub claims: T,
    pub token: String,
}

impl<T> Factory<T>
where
    T: Claims,
{
    pub fn new(claims: T, token: String) -> Self {
        Self { claims, token }
    }
}

pub enum TokenResponse<T>
where
    T: Claims,
{
    Access(Factory<T>),
    Refresh(Factory<T>),
    Session(Factory<T>),
    Reauth(Factory<T>),
}

impl<T> Display for TokenResponse<T>
where
    T: Claims,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TokenResponse::Access(factory) => write!(f, "Access: {}", factory.token),
            TokenResponse::Refresh(factory) => write!(f, "Refresh: {}", factory.token),
            TokenResponse::Session(factory) => write!(f, "Session: {}", factory.token),
            TokenResponse::Reauth(factory) => write!(f, "Reauth: {}", factory.token),
        }
    }
}

impl<T> TokenResponse<T>
where
    T: Claims,
{
    pub fn factory(&self) -> &Factory<T> {
        match self {
            TokenResponse::Access(factory) => factory,
            TokenResponse::Refresh(factory) => factory,
            TokenResponse::Session(factory) => factory,
            TokenResponse::Reauth(factory) => factory,
        }
    }

    pub fn claims(&self) -> &T {
        &self.factory().claims
    }

    pub fn token(&self) -> &str {
        self.factory().token.as_str()
    }
}
