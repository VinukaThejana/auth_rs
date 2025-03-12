use crate::{model::user::UserDetails, util::now};
use serde::{Deserialize, Serialize};

pub trait Claims {
    fn sub(&self) -> &str;
    fn jti(&self) -> &str;
    fn rjti(&self) -> &str;
    fn iat(&self) -> usize;
    fn exp(&self) -> usize;
    fn nbf(&self) -> usize;
    fn custom(&self) -> Option<&str>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrimaryClaims {
    pub sub: String,
    pub jti: String,
    pub rjti: String,
    pub exp: usize,
    pub iat: usize,
    pub nbf: usize,
    pub custom: Option<String>,
}

impl PrimaryClaims {
    pub fn new(
        sub: String,
        exp: usize,
        jti: Option<String>,
        rjti: Option<String>,
        custom: Option<String>,
    ) -> Self {
        let jti = jti.unwrap_or_else(|| ulid::Ulid::new().to_string());
        let now = now();
        let rjti = rjti.unwrap_or(jti.clone());

        Self {
            sub,
            jti,
            rjti,
            exp: now + exp,
            iat: now,
            nbf: now,
            custom,
        }
    }
}

impl Claims for PrimaryClaims {
    fn sub(&self) -> &str {
        &self.sub
    }

    fn jti(&self) -> &str {
        &self.jti
    }

    fn rjti(&self) -> &str {
        &self.rjti
    }

    fn iat(&self) -> usize {
        self.iat
    }

    fn exp(&self) -> usize {
        self.exp
    }

    fn nbf(&self) -> usize {
        self.nbf
    }

    fn custom(&self) -> Option<&str> {
        self.custom.as_deref()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtendedClaims {
    #[serde(flatten)]
    pub primary: PrimaryClaims,

    #[serde(flatten)]
    pub user: UserDetails,
}

impl ExtendedClaims {
    pub fn new(user: &UserDetails, exp: usize) -> Self {
        Self {
            primary: PrimaryClaims::new(user.id.clone(), exp, None, None, None),
            user: user.into(),
        }
    }
}

impl Claims for ExtendedClaims {
    fn sub(&self) -> &str {
        &self.primary.sub
    }

    fn jti(&self) -> &str {
        &self.primary.jti
    }

    fn rjti(&self) -> &str {
        &self.primary.rjti
    }

    fn iat(&self) -> usize {
        self.primary.iat
    }

    fn exp(&self) -> usize {
        self.primary.exp
    }

    fn nbf(&self) -> usize {
        self.primary.nbf
    }

    fn custom(&self) -> Option<&str> {
        None
    }
}
