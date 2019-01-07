use actix_web::{error::ResponseError, HttpResponse};
use argon2::Error as ArgonError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use failure::Fail;
use jsonwebtoken::errors::Error as JWTError;

#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "Internal Server Error")]
    InternalServerError,

    #[fail(display = "BadRequest: {}", _0)]
    BadRequest(String),

    #[fail(display = "Anauthorized")]
    Unauthorized,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().finish(),
        }
    }
}

impl From<DieselError> for ServiceError {
    fn from(error: DieselError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError
            }
            _ => ServiceError::InternalServerError,
        }
    }
}

impl From<ArgonError> for ServiceError {
    fn from(error: ArgonError) -> ServiceError {
        match error {
            ArgonError::PwdTooShort | ArgonError::PwdTooLong => {
                ServiceError::BadRequest(format!("password length error: {}", error))
            }
            _ => ServiceError::InternalServerError,
        }
    }
}

impl From<JWTError> for ServiceError {
    fn from(_error: JWTError) -> ServiceError {
        ServiceError::InternalServerError
    }
}
