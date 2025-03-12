use crate::{
    config::{ENV, state::AppState},
    token::{
        TokenType,
        claims::PrimaryClaims,
        error::TokenError,
        params::TokenParams,
        response::{Factory, TokenResponse},
        traits::Token,
    },
};

pub struct ReAuth {
    state: AppState,
    pub user_id: Option<String>,
}

impl ReAuth {
    pub fn default(state: AppState) -> Self {
        Self {
            state,
            user_id: None,
        }
    }

    pub fn new(state: AppState, user_id: String) -> Self {
        Self {
            state,
            user_id: Some(user_id),
        }
    }

    fn user_id(&self) -> &str {
        self.user_id
            .as_deref()
            .expect("user id is required to create a reauth token")
    }

    fn claims(&self) -> PrimaryClaims {
        PrimaryClaims::new(self.user_id().to_owned(), self.exp(), None, None, None)
    }
}

impl Token<PrimaryClaims> for ReAuth {
    fn state(&self) -> AppState {
        self.state.clone()
    }

    fn public_key(&self) -> &[u8] {
        &ENV.reauth_token_public_key
    }
    fn private_key(&self) -> &[u8] {
        &ENV.reauth_token_private_key
    }
    fn exp(&self) -> usize {
        ENV.reauth_token_expiration
    }

    async fn create(&self, _: TokenParams) -> Result<TokenResponse<PrimaryClaims>, TokenError> {
        let claims = self.claims();
        let token = self.generate(&claims)?;
        Ok(TokenResponse::Reauth(Factory::new(claims, token)))
    }

    async fn verify(&self, token: &str, _: TokenType) -> Result<PrimaryClaims, TokenError>
    where
        PrimaryClaims: Send,
    {
        self.decode(token)
    }
}
