use crate::{
    config::{ENV, state::AppState},
    model::user::UserDetails,
    token::{
        TokenType,
        claims::ExtendedClaims,
        error::TokenError,
        params::TokenParams,
        response::{Factory, TokenResponse},
        traits::Token,
    },
};

pub struct Session {
    pub state: AppState,
    pub user: Option<UserDetails>,
}

impl Session {
    pub fn default(state: AppState) -> Self {
        Self { state, user: None }
    }

    pub fn new(state: AppState, user: UserDetails) -> Self {
        Self {
            state,
            user: Some(user),
        }
    }

    fn user(&self) -> &UserDetails {
        self.user
            .as_ref()
            .expect("user details are required to create the session token")
    }
}

impl Token<ExtendedClaims> for Session {
    fn state(&self) -> AppState {
        self.state.clone()
    }

    fn public_key(&self) -> &[u8] {
        &ENV.session_token_public_key
    }
    fn private_key(&self) -> &[u8] {
        &ENV.session_token_private_key
    }
    fn exp(&self) -> usize {
        ENV.session_token_expiration
    }

    async fn create(&self, _: TokenParams) -> Result<TokenResponse<ExtendedClaims>, TokenError> {
        let claims = ExtendedClaims::new(self.user(), self.exp());
        let token = self.generate(&claims)?;
        Ok(TokenResponse::Session(Factory::new(claims, token)))
    }

    async fn verify(&self, token: &str, _: TokenType) -> Result<ExtendedClaims, TokenError>
    where
        ExtendedClaims: Send,
    {
        self.decode(token)
    }
}
