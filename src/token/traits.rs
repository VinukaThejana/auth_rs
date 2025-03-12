use super::{
    TokenType, claims::Claims, error::TokenError, params::TokenParams, response::TokenResponse,
};
use crate::config::state::AppState;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub trait Token<T>
where
    T: for<'a> Serialize,
    T: for<'a> Deserialize<'a>,
    T: Claims,
    Self: Send + Sync,
{
    fn state(&self) -> AppState;

    fn public_key(&self) -> &[u8];
    fn private_key(&self) -> &[u8];
    fn exp(&self) -> usize;

    fn encode_rsa(&self, key: &[u8]) -> EncodingKey {
        EncodingKey::from_rsa_pem(key).unwrap()
    }
    fn decode_rsa(&self, key: &[u8]) -> DecodingKey {
        DecodingKey::from_rsa_pem(key).unwrap()
    }

    fn generate(&self, claims: &T) -> Result<String, TokenError> {
        jsonwebtoken::encode(
            &Header::new(Algorithm::RS256),
            claims,
            &self.encode_rsa(self.private_key()),
        )
        .map_err(|err| TokenError::Creation(err.into()))
    }
    fn decode(&self, token: &str) -> Result<T, TokenError> {
        let claims = jsonwebtoken::decode::<T>(
            token,
            &self.decode_rsa(self.public_key()),
            &Validation::new(Algorithm::RS256),
        )
        .map_err(|err| TokenError::Validation(err.into()))?
        .claims;

        Ok(claims)
    }

    fn create(
        &self,
        params: TokenParams,
    ) -> impl Future<Output = Result<TokenResponse<T>, TokenError>> + Send;

    fn verify(
        &self,
        token: &str,
        token_type: TokenType,
    ) -> impl Future<Output = Result<T, TokenError>> + Send
    where
        T: Send,
    {
        async move {
            let claims = self.decode(token)?;

            let mut conn = self
                .state()
                .get_redis_conn()
                .await
                .map_err(TokenError::Other)?;

            let value: Option<String> = redis::cmd("GET")
                .arg(token_type.get_key(claims.jti()))
                .query_async(&mut conn)
                .await
                .map_err(|err| TokenError::Other(err.into()))?;
            let value =
                value.ok_or_else(|| TokenError::Validation(anyhow::anyhow!("token not found")))?;

            match token_type {
                TokenType::Access => {
                    if value != claims.sub() {
                        return Err(TokenError::Validation(anyhow::anyhow!(
                            "access token is invalid"
                        )));
                    }
                }
                TokenType::Refresh => {
                    if value.is_empty() {
                        return Err(TokenError::Validation(anyhow::anyhow!(
                            "refresh token is invalid"
                        )));
                    }
                }
                TokenType::Session => {
                    unimplemented!("session token verification can be done with self.decode()")
                }
                TokenType::ReAuth => {
                    unimplemented!("reauht token verification can be done with self.decode()")
                }
            }

            Ok(claims)
        }
    }
}
