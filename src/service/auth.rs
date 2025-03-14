use crate::{
    auth_proto::{
        ChangeEmailRequest, ChangeEmailResponse, ChangePasswordRequest, ChangePasswordResponse,
        ChangeUsernameRequest, ChangeUsernameResponse, DeleteRequest, DeleteResponse,
        ForgotPasswordRequest, ForgotPasswordResponse, LoginRequest, LoginResponse, LogoutRequest,
        LogoutResponse, ReauthTokenRequest, ReauthTokenResponse, RefreshRequest, RefreshResponse,
        RegisterRequest, RegisterResponse, ResetPasswordResponse,
        SendEmailVerificationForNewEmailRequest, SendEmailVerificationForNewEmailResponse,
        SendEmailVerificationRequest, SendEmailVerificationResponse, Token,
        VerifyEmailTokenRequest, VerifyEmailTokenResponse, VerifyForgotPasswordTokenRequest,
        VerifyForgotPasswordTokenResponse, VerifyTokenRequest, VerifyTokenResponse,
        auth_service_server::AuthService, login_response::Tokens,
    },
    config::{ENV, state::AppState},
    database,
    error::AppError,
    model::user::{CreateUserReq, UserDetails},
    token::{claims::Claims, service::factory, types::refresh::Refresh},
};
use tonic::{Request, Response, Status};
use validator::Validate;

#[derive(Clone)]
pub struct Service {
    state: AppState,
}

impl Service {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl AuthService for Service {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let payload: CreateUserReq = request.into_inner().into();
        payload
            .validate()
            .map_err(AppError::from_validation_errors)?;

        database::user::create(
            &self.state.db,
            "email",
            &payload.email,
            &payload.username,
            &payload.name,
            Some(&payload.password),
            None,
        )
        .await
        .map_err(AppError::from_database_error)?;

        Ok(Response::new(RegisterResponse {}))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let request = request.into_inner();
        let state = self.state.clone();

        let user = database::user::get_by_credential(&state.db, &request.credential)
            .await
            .map_err(AppError::from_database_error)?;
        let password = user
            .password
            .clone()
            .ok_or(AppError::InvalidProvider(anyhow::anyhow!(
                "canot login with password, you have used another login provider"
            )))?;

        bcrypt::verify(&request.password, &password)
            .map_err(|err| AppError::IncorrectCredentials(err.into()))?;

        if user.is_two_factor_enabled {
            let otp = request.otp.ok_or(AppError::OTPRequired(anyhow::anyhow!(
                "OTP is required to login"
            )))?;

            let mut conn = self.state.get_redis_conn().await.map_err(AppError::Other)?;
            let value: Option<String> = redis::cmd("GET")
                .arg(format!("{}:twofactor:otp:{}", &*ENV.redis_schema, &otp))
                .query_async(&mut conn)
                .await
                .map_err(|err| AppError::Other(err.into()))?;
            value.ok_or(AppError::OTPInvalid(anyhow::anyhow!("OTP is invalid")))?;
        }

        let user: UserDetails = user.into();
        let tokens = factory(self.state.clone(), &user).await?;

        let rjti = tokens.refresh.claims.jti.clone();

        tokio::spawn(async move {
            if let Err(err) = database::session::create(
                &state.db,
                &rjti,
                &user.id,
                &request.ip_address,
                request.user_agent.as_deref().unwrap_or(""),
            )
            .await
            {
                if let Err(err) = Refresh::default(state.clone()).delete(&rjti).await {
                    log::error!(
                        "{}",
                        anyhow::Error::new(err)
                            .context("failed to delete refresh token from redis")
                    )
                }

                log::error!(
                    "{}",
                    anyhow::Error::new(err)
                        .context("failed to create the session record in the database")
                )
            }

            if let Err(err) =
                database::session::delete_expired_user_sessions(&state.db, &user.id).await
            {
                log::error!(
                    "{}",
                    anyhow::Error::new(err)
                        .context("failed to delete expired user sessions from the database")
                )
            }
        });

        Ok(Response::new(LoginResponse {
            tokens: Some(Tokens {
                refresh: Some(Token {
                    token: tokens.refresh.token,
                    expires: tokens.refresh.claims.exp() as u64,
                }),
                access: Some(Token {
                    token: tokens.access.token,
                    expires: tokens.access.claims.exp() as u64,
                }),
                session: Some(Token {
                    token: tokens.session.token,
                    expires: tokens.session.claims.exp() as u64,
                }),
            }),
        }))
    }

    async fn refresh(
        &self,
        request: Request<RefreshRequest>,
    ) -> Result<Response<RefreshResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }

    async fn reauth_token(
        &self,
        request: Request<ReauthTokenRequest>,
    ) -> Result<Response<ReauthTokenResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }

    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }

    async fn send_email_verification(
        &self,
        request: Request<SendEmailVerificationRequest>,
    ) -> Result<Response<SendEmailVerificationResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }

    async fn send_email_verification_for_new_email(
        &self,
        request: Request<SendEmailVerificationForNewEmailRequest>,
    ) -> Result<Response<SendEmailVerificationForNewEmailResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }

    async fn verify_token(
        &self,
        request: Request<VerifyTokenRequest>,
    ) -> Result<Response<VerifyTokenResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }

    async fn verify_email_token(
        &self,
        request: Request<VerifyEmailTokenRequest>,
    ) -> Result<Response<VerifyEmailTokenResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }

    async fn verify_forgot_password_token(
        &self,
        request: Request<VerifyForgotPasswordTokenRequest>,
    ) -> Result<Response<VerifyForgotPasswordTokenResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }

    async fn forgot_password(
        &self,
        request: Request<ForgotPasswordRequest>,
    ) -> Result<Response<ForgotPasswordResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }

    async fn reset_password(
        &self,
        request: Request<ResetPasswordResponse>,
    ) -> Result<Response<ResetPasswordResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }

    async fn change_email(
        &self,
        request: Request<ChangeEmailRequest>,
    ) -> Result<Response<ChangeEmailResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }

    async fn change_username(
        &self,
        request: Request<ChangeUsernameRequest>,
    ) -> Result<Response<ChangeUsernameResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }

    async fn change_password(
        &self,
        request: Request<ChangePasswordRequest>,
    ) -> Result<Response<ChangePasswordResponse>, Status> {
        let _request = request.into_inner();

        todo!()
    }
}
