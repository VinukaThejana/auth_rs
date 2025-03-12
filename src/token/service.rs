use super::{
    claims::{Claims, ExtendedClaims, PrimaryClaims},
    params::TokenParams,
    response::{Factory, TokenResponse},
    traits::Token,
    types::refresh::Refresh,
};
use crate::{
    config::state::AppState,
    error::AppError,
    model::user::UserDetails,
    token::types::{access::Access, session::Session},
};
use serde::{Deserialize, Serialize};

pub async fn create_token<T, U>(token: T, params: TokenParams) -> Result<TokenResponse<U>, AppError>
where
    T: Token<U>,
    U: for<'a> Serialize,
    U: for<'a> Deserialize<'a>,
    U: Claims,
    U: Send + Sync,
{
    token
        .create(params)
        .await
        .map_err(AppError::from_token_error)
}

pub struct TokenFactory {
    pub refresh: Factory<PrimaryClaims>,
    pub access: Factory<PrimaryClaims>,
    pub session: Factory<ExtendedClaims>,
}

pub async fn factory(state: AppState, user: &UserDetails) -> Result<TokenFactory, AppError> {
    let refresh = create_token(
        Refresh::new(state.clone(), &user.id),
        TokenParams::default(),
    )
    .await?;
    let claims = refresh.claims().clone();
    let access_token_jti = claims
        .custom
        .expect("refresh token must return the access token jti");

    let access = create_token(
        Access::new(state.clone(), &user.id),
        TokenParams::default()
            .with_ajti(access_token_jti.clone())
            .with_rjti(claims.jti),
    )
    .await?;

    let session = create_token(
        Session::new(state.clone(), user.into()),
        TokenParams::default(),
    )
    .await?;

    Ok(TokenFactory {
        refresh: refresh.factory().clone(),
        access: access.factory().clone(),
        session: session.factory().clone(),
    })
}
