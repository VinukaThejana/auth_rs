use sea_orm::{DbErr, RuntimeErr};
use std::fmt::{Display, Formatter, Result};
use tonic::{Code, Status};
use validator::ValidationErrors;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    Database(#[from] DbErr),
    NotFound(#[source] anyhow::Error),
    BadRequest(#[source] anyhow::Error),
    UniqueViolation(#[source] anyhow::Error),
    Unauthorized(#[source] anyhow::Error),
    InvalidProvider(#[source] anyhow::Error),
    OTPRequired(#[source] anyhow::Error),
    OTPInvalid(#[source] anyhow::Error),
    IncorrectCredentials(#[source] anyhow::Error),
    Validation(#[from] ValidationErrors),
    Other(#[from] anyhow::Error),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Database(err) => write!(f, "{:?}", err),
            Self::NotFound(err) => write!(f, "{:?}", err),
            Self::BadRequest(err) => write!(f, "{:?}", err),
            Self::UniqueViolation(err) => write!(f, "{:?}", err),
            Self::Unauthorized(err) => write!(f, "{:?}", err),
            Self::InvalidProvider(err) => write!(f, "{:?}", err),
            Self::OTPRequired(err) => write!(f, "{:?}", err),
            Self::OTPInvalid(err) => write!(f, "{:?}", err),
            Self::IncorrectCredentials(err) => write!(f, "{:?}", err),
            Self::Validation(errs) => {
                let message =
                    errs.field_errors()
                        .iter()
                        .fold(String::new(), |acc, (field, errors)| {
                            let errors = errors
                                .iter()
                                .map(|err| {
                                    err.message
                                        .as_ref()
                                        .map(|msg| msg.to_string())
                                        .unwrap_or_else(|| String::from("invalid value"))
                                })
                                .collect::<Vec<String>>();

                            let error = errors.first();
                            if error.is_none() {
                                return acc;
                            }
                            let error = error.unwrap();

                            if acc.is_empty() {
                                format!(r#""{}": "{}""#, field, error)
                            } else {
                                format!(r#"{}, "{}": "{}""#, acc, field, error)
                            }
                        });

                write!(f, "{}", message)
            }
            Self::Other(err) => write!(f, "{:?}", err),
        }
    }
}

impl AppError {
    pub fn from_validation_errors(error: ValidationErrors) -> Self {
        Self::Validation(error)
    }

    pub fn from_generic_error<E>(error: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::Other(anyhow::Error::new(error))
    }

    pub fn from_database_error(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(err) => Self::NotFound(anyhow::anyhow!(err)),
            err => {
                if is_unique_violation(&err) {
                    Self::UniqueViolation(err.into())
                } else {
                    Self::Database(err)
                }
            }
        }
    }
}

fn is_unique_violation(err: &DbErr) -> bool {
    match err {
        DbErr::Query(RuntimeErr::SqlxError(error)) => {
            if let Some(db_error) = error.as_database_error() {
                if let Some(code) = db_error.code() {
                    code == "23505"
                } else {
                    false
                }
            } else {
                false
            }
        }
        _ => false,
    }
}

impl From<AppError> for Status {
    fn from(value: AppError) -> Self {
        match value {
            AppError::Database(error) => {
                log::error!("[database_error]: {:?}", error);
                Status::new(Code::Internal, error.to_string())
            }
            AppError::NotFound(error) => {
                log::error!("[not_found]: {:?}", error);
                Status::new(Code::NotFound, error.to_string())
            }
            AppError::BadRequest(error) => {
                log::error!("[bad_request]: {:?}", error);
                Status::new(Code::InvalidArgument, error.to_string())
            }
            AppError::UniqueViolation(error) => {
                log::error!("[unique_violation]: {:?}", error);
                Status::new(Code::AlreadyExists, error.to_string())
            }
            AppError::Unauthorized(error) => {
                log::error!("[unauthorized]: {:?}", error);
                Status::new(Code::PermissionDenied, error.to_string())
            }
            AppError::InvalidProvider(error) => {
                log::error!("[invalid_provider]: {:?}", error);
                Status::new(Code::InvalidArgument, error.to_string())
            }
            AppError::OTPRequired(error) => {
                log::error!("[otp_required]: {:?}", error);
                Status::new(Code::FailedPrecondition, error.to_string())
            }
            AppError::OTPInvalid(error) => {
                log::error!("[otp_invalid]: {:?}", error);
                Status::new(Code::InvalidArgument, error.to_string())
            }
            AppError::IncorrectCredentials(error) => {
                log::error!("[incorrect_credentials]: {:?}", error);
                Status::new(Code::PermissionDenied, error.to_string())
            }
            AppError::Validation(error) => {
                log::error!("validation_error: {:?}", error);
                Status::new(Code::FailedPrecondition, error.to_string())
            }
            AppError::Other(error) => {
                log::error!("[other_error]: {:?}", error);
                Status::new(Code::Internal, error.to_string())
            }
        }
    }
}
