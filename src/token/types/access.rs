use crate::{
    config::{ENV, state::AppState},
    token::{
        TokenType,
        claims::{Claims, PrimaryClaims},
        error::TokenError,
        params::TokenParams,
        response::{Factory, TokenResponse},
        traits::Token,
    },
};

pub struct Access {
    pub state: AppState,
    pub user_id: Option<String>,
}

impl Access {
    pub fn default(state: AppState) -> Self {
        Self {
            state,
            user_id: None,
        }
    }

    pub fn new(state: AppState, user_id: &str) -> Self {
        Self {
            state,
            user_id: Some(user_id.to_owned()),
        }
    }

    fn user_id(&self) -> &str {
        self.user_id
            .as_deref()
            .expect("user id is required to create an access token")
    }

    fn claims(&self, params: TokenParams) -> PrimaryClaims {
        PrimaryClaims::new(
            self.user_id().to_owned(),
            self.exp(),
            params.ajti,
            params.rjti,
            None,
        )
    }
}

impl Token<PrimaryClaims> for Access {
    fn state(&self) -> AppState {
        self.state.clone()
    }

    fn public_key(&self) -> &[u8] {
        &ENV.access_token_public_key
    }
    fn private_key(&self) -> &[u8] {
        &ENV.access_token_private_key
    }
    fn exp(&self) -> usize {
        ENV.access_token_expiration
    }

    async fn create(
        &self,
        params: TokenParams,
    ) -> Result<TokenResponse<PrimaryClaims>, TokenError> {
        let ajti = params.ajti.clone().unwrap_or(ulid::Ulid::new().to_string());
        let rjti = params
            .rjti
            .expect("refresh token jti is required to create an access token");
        let claims = self.claims(
            TokenParams::default()
                .with_rjti(rjti.clone())
                .with_ajti(ajti.clone()),
        );
        let token = self.generate(&claims)?;
        let response = TokenResponse::Access(Factory::new(claims, token));

        if params.ajti.is_some() {
            return Ok(response);
        }

        let mut conn = self
            .state()
            .get_redis_conn()
            .await
            .map_err(TokenError::Other)?;

        let value: Option<String> = redis::cmd("GET")
            .arg(TokenType::Refresh.get_key(&rjti))
            .query_async(&mut conn)
            .await
            .map_err(|err| TokenError::Other(err.into()))?;

        let _: () = redis::pipe()
            .cmd("DEL")
            .arg(if let Some(ref v) = value {
                TokenType::Access.get_key(v)
            } else {
                String::from("no_key")
            })
            .ignore()
            .cmd("SET")
            .arg(TokenType::Refresh.get_key(&rjti))
            .arg(&ajti)
            .arg("KEEPTTL")
            .ignore()
            .cmd("SET")
            .arg(TokenType::Access.get_key(&ajti))
            .arg(self.user_id())
            .arg("EX")
            .arg(self.exp())
            .ignore()
            .query_async(&mut conn)
            .await
            .map_err(|err| TokenError::Other(err.into()))?;

        Ok(response)
    }
}

impl Access {
    pub async fn refresh(&self, rjti: &str) -> Result<String, TokenError> {
        let claims = self.claims(TokenParams::default().with_rjti(rjti.to_owned()));
        let token = self.generate(&claims)?;

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

        let _: () = redis::pipe()
            .cmd("DEL")
            .arg(if let Some(ref v) = value {
                TokenType::Access.get_key(v)
            } else {
                String::from("no_key")
            })
            .ignore()
            .cmd("SET")
            .arg(TokenType::Refresh.get_key(rjti))
            .arg(claims.jti())
            .arg("KEEPTTL")
            .ignore()
            .cmd("SET")
            .arg(TokenType::Access.get_key(claims.jti()))
            .arg(self.user_id())
            .arg("EX")
            .arg(self.exp())
            .ignore()
            .query_async(&mut conn)
            .await
            .map_err(|err| TokenError::Other(err.into()))?;

        Ok(token)
    }
}
