use crate::admin_proto::{
    CreateAdminRequest, CreateAdminResponse, CreateApiKeyRequest, CreateApiKeyResponse,
    DeleteAdminRequest, DeleteAdminResponse, DeleteApiKeyRequest, DeleteApiKeyResponse,
    ListApiKeysRequest, ListApiKeysResponse, SendEmailResponse,
};
use crate::admin_proto::{SendEmailRequest, admin_service_server::AdminService};
use crate::config::ENV;
use crate::config::state::AppState;
use crate::database;
use crate::error::AppError;
use crate::model::admin::{CreateAdminReq, DeleteAdminReq};
use crate::model::api::{CreateApiKeyReq, DeleteApiKeyReq, ListApiKeysReq};
use crate::template::email::send_otp;
use crate::util::{generate_otp, validate_otp};
use resend_rs::types::CreateEmailBaseOptions;
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

fn otp_key(email: &str) -> String {
    format!("{}:admin:verification:{}", &ENV.redis_schema, email)
}

#[tonic::async_trait]
impl AdminService for Service {
    async fn send_email(
        &self,
        request: Request<SendEmailRequest>,
    ) -> Result<Response<SendEmailResponse>, Status> {
        let request = request.into_inner();
        let otp = generate_otp();

        let mut conn = self.state.get_redis_conn().await.map_err(AppError::Other)?;
        let _: () = redis::cmd("SET")
            .arg(otp_key(&request.email))
            .arg(&otp)
            .arg("EX")
            .arg(60 * 60)
            .query_async(&mut conn)
            .await
            .map_err(|err| AppError::Other(err.into()))?;

        let email = CreateEmailBaseOptions::new(
            &*ENV.resend_email,
            [&request.email],
            format!("[{}] Admin verification from auth_rs", &otp),
        )
        .with_html(
            send_otp(&otp, "to verify your email")
                .into_string()
                .as_str(),
        );

        self.state
            .resend
            .emails
            .send(email)
            .await
            .map_err(|err| AppError::Other(err.into()))?;

        Ok(Response::new(SendEmailResponse {}))
    }

    async fn create_admin(
        &self,
        request: Request<CreateAdminRequest>,
    ) -> Result<Response<CreateAdminResponse>, Status> {
        let request: CreateAdminReq = request.into_inner().into();
        request
            .validate()
            .map_err(AppError::from_validation_errors)?;
        validate_otp(self.state.clone(), &otp_key(&request.email), &request.otp).await?;

        database::admin::create(&self.state.db, &request.email, &request.description)
            .await
            .map_err(AppError::from_database_error)?;

        Ok(Response::new(CreateAdminResponse {}))
    }

    async fn delete_admin(
        &self,
        request: Request<DeleteAdminRequest>,
    ) -> Result<Response<DeleteAdminResponse>, Status> {
        let request: DeleteAdminReq = request.into_inner().into();
        request
            .validate()
            .map_err(AppError::from_validation_errors)?;
        validate_otp(self.state.clone(), &otp_key(&request.email), &request.otp).await?;

        database::admin::delete(&self.state.db, &request.email)
            .await
            .map_err(AppError::from_database_error)?;

        Ok(Response::new(DeleteAdminResponse {}))
    }

    async fn list_api_keys(
        &self,
        request: Request<ListApiKeysRequest>,
    ) -> Result<Response<ListApiKeysResponse>, Status> {
        let request: ListApiKeysReq = request.into_inner().into();
        request
            .validate()
            .map_err(AppError::from_validation_errors)?;
        validate_otp(self.state.clone(), &otp_key(&request.email), &request.otp).await?;

        let admin = database::admin::get_by_email(&self.state.db, &request.email)
            .await
            .map_err(AppError::from_database_error)?;

        let api_keys = database::api_key::list(&self.state.db, &admin.id)
            .await
            .map_err(AppError::from_database_error)?;

        Ok(Response::new(api_keys.into()))
    }

    async fn create_api_key(
        &self,
        request: Request<CreateApiKeyRequest>,
    ) -> Result<Response<CreateApiKeyResponse>, Status> {
        let request: CreateApiKeyReq = request.into_inner().into();
        request
            .validate()
            .map_err(AppError::from_validation_errors)?;
        validate_otp(self.state.clone(), &otp_key(&request.email), &request.otp).await?;

        let admin = database::admin::get_by_email(&self.state.db, &request.email)
            .await
            .map_err(AppError::from_database_error)?;

        let api = database::api_key::create(&self.state.db, &admin.id, &request.description)
            .await
            .map_err(AppError::from_database_error)?;

        Ok(Response::new(CreateApiKeyResponse {
            api_key: Some(api.key),
            api_secret: Some(api.secret),
        }))
    }

    async fn delete_api_key(
        &self,
        request: Request<DeleteApiKeyRequest>,
    ) -> Result<Response<DeleteApiKeyResponse>, Status> {
        let request: DeleteApiKeyReq = request.into_inner().into();
        request
            .validate()
            .map_err(AppError::from_validation_errors)?;
        validate_otp(self.state.clone(), &otp_key(&request.email), &request.otp).await?;

        database::api_key::delete(&self.state.db, &request.key)
            .await
            .map_err(AppError::from_database_error)?;

        Ok(Response::new(DeleteApiKeyResponse {}))
    }
}
