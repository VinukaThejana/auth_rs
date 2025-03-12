use crate::{
    config::{ENV, state::AppState},
    database,
    token::{
        TokenType,
        claims::{Claims, PrimaryClaims},
        error::TokenError,
        params::TokenParams,
        response::{Factory, TokenResponse},
        traits::Token,
    },
};
use ulid::Ulid;

pub struct Refresh {
    pub state: AppState,
    pub user_id: Option<String>,
}

impl Refresh {
    pub fn default(state: AppState) -> Self {
        Self {
            state,
            user_id: None,
        }
    }

    pub fn new(state: AppState, user_id: &str) -> Self {
        Self {
            state,
            user_id: Some(user_id.to_string()),
        }
    }

    fn user_id(&self) -> &str {
        self.user_id
            .as_deref()
            .expect("user id is required to create a refresh token")
    }
}

impl Token<PrimaryClaims> for Refresh {
    fn state(&self) -> AppState {
        self.state.clone()
    }

    fn public_key(&self) -> &[u8] {
        &ENV.refresh_token_public_key
    }
    fn private_key(&self) -> &[u8] {
        &ENV.refresh_token_private_key
    }
    fn exp(&self) -> usize {
        ENV.refresh_token_expiration
    }

    async fn create(&self, _: TokenParams) -> Result<TokenResponse<PrimaryClaims>, TokenError> {
        let ajti = Ulid::new().to_string();
        let claims = PrimaryClaims::new(
            self.user_id().to_owned(),
            self.exp(),
            None,
            None,
            Some(ajti.clone()),
        );
        let token = self.generate(&claims)?;

        let mut conn = self
            .state()
            .get_redis_conn()
            .await
            .map_err(TokenError::Other)?;

        let _: () = redis::pipe()
            .cmd("SET")
            .arg(TokenType::Refresh.get_key(claims.jti()))
            .arg(&ajti)
            .arg("EX")
            .arg(self.exp())
            .ignore()
            .arg("SET")
            .arg(TokenType::Access.get_key(&ajti))
            .arg(self.user_id())
            .arg("EX")
            .arg(ENV.access_token_expiration)
            .query_async(&mut conn)
            .await
            .map_err(|err| TokenError::Other(err.into()))?;

        Ok(TokenResponse::Refresh(Factory::new(claims, token)))
    }
}

impl Refresh {
    pub async fn delete(&self, rjti: &str) -> Result<(), TokenError> {
        database::session::delete(&self.state.db, rjti)
            .await
            .map_err(|err| TokenError::Other(err.into()))?;

        let mut conn = self
            .state()
            .get_redis_conn()
            .await
            .map_err(TokenError::Other)?;

        let value: Option<String> = redis::cmd("GET")
            .arg(TokenType::Refresh.get_key(rjti))
            .query_async(&mut conn)
            .await
            .map_err(|err| TokenError::Other(err.into()))?;
        let value = value
            .ok_or_else(|| TokenError::Validation(anyhow::anyhow!("token not found in redis")))?;

        let _: () = redis::pipe()
            .cmd("DEL")
            .arg(TokenType::Refresh.get_key(rjti))
            .ignore()
            .cmd("DEL")
            .arg(TokenType::Access.get_key(&value))
            .ignore()
            .query_async(&mut conn)
            .await
            .map_err(|err| TokenError::Other(err.into()))?;

        Ok(())
    }
}
