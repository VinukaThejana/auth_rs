use crate::{
    auth_proto::{
        ChangeEmailRequest, ChangeEmailResponse, ChangePasswordRequest, ChangePasswordResponse,
        ChangeUsernameRequest, ChangeUsernameResponse, DeleteRequest, DeleteResponse,
        ForgotPasswordRequest, ForgotPasswordResponse, LoginRequest, LoginResponse, LogoutRequest,
        LogoutResponse, ReauthTokenRequest, ReauthTokenResponse, RefreshRequest, RefreshResponse,
        RegisterRequest, RegisterResponse, ResetPasswordResponse,
        SendEmailVerificationForNewEmailRequest, SendEmailVerificationForNewEmailResponse,
        SendEmailVerificationRequest, SendEmailVerificationResponse, VerifyEmailTokenRequest,
        VerifyEmailTokenResponse, VerifyForgotPasswordTokenRequest,
        VerifyForgotPasswordTokenResponse, VerifyTokenRequest, VerifyTokenResponse,
        auth_service_server::AuthService,
    },
    config::state::{self, AppState},
    database,
    error::AppError,
    model::user::CreateUserReq,
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

        todo!()
    }

    async fn refresh(
        &self,
        request: Request<RefreshRequest>,
    ) -> Result<Response<RefreshResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }

    async fn reauth_token(
        &self,
        request: Request<ReauthTokenRequest>,
    ) -> Result<Response<ReauthTokenResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }

    async fn logout(
        &self,
        request: Request<LogoutRequest>,
    ) -> Result<Response<LogoutResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }

    async fn send_email_verification(
        &self,
        request: Request<SendEmailVerificationRequest>,
    ) -> Result<Response<SendEmailVerificationResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }

    async fn send_email_verification_for_new_email(
        &self,
        request: Request<SendEmailVerificationForNewEmailRequest>,
    ) -> Result<Response<SendEmailVerificationForNewEmailResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }

    async fn verify_token(
        &self,
        request: Request<VerifyTokenRequest>,
    ) -> Result<Response<VerifyTokenResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }

    async fn verify_email_token(
        &self,
        request: Request<VerifyEmailTokenRequest>,
    ) -> Result<Response<VerifyEmailTokenResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }

    async fn verify_forgot_password_token(
        &self,
        request: Request<VerifyForgotPasswordTokenRequest>,
    ) -> Result<Response<VerifyForgotPasswordTokenResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }

    async fn forgot_password(
        &self,
        request: Request<ForgotPasswordRequest>,
    ) -> Result<Response<ForgotPasswordResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }

    async fn reset_password(
        &self,
        request: Request<ResetPasswordResponse>,
    ) -> Result<Response<ResetPasswordResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }

    async fn change_email(
        &self,
        request: Request<ChangeEmailRequest>,
    ) -> Result<Response<ChangeEmailResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }

    async fn change_username(
        &self,
        request: Request<ChangeUsernameRequest>,
    ) -> Result<Response<ChangeUsernameResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }

    async fn change_password(
        &self,
        request: Request<ChangePasswordRequest>,
    ) -> Result<Response<ChangePasswordResponse>, Status> {
        let request = request.into_inner();

        todo!()
    }
}
