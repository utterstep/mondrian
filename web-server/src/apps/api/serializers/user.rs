use std::convert::TryFrom;

use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, errors::Result as JWTResult, Header, Validation};
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

use mondrian_models::users::User;

use crate::errors::ServiceError;

#[derive(Debug, Serialize)]
pub struct UserInfo {
    id: i32,
    first_name: String,
    last_name: String,
    middle_name: String,
    email: String,
    phone: String,
    superuser: bool,
}

#[derive(Debug)]
pub struct SuperuserInfo(pub UserInfo);

impl TryFrom<UserInfo> for SuperuserInfo {
    type Error = SuperuserConversionError;

    fn try_from(user_info: UserInfo) -> Result<Self, Self::Error> {
        if user_info.superuser {
            Ok(Self(user_info))
        } else {
            Err(SuperuserConversionError::NotSuperuser)
        }
    }
}

pub enum SuperuserConversionError {
    NotSuperuser,
}

impl Into<ServiceError> for SuperuserConversionError {
    fn into(self) -> ServiceError {
        match self {
            SuperuserConversionError::NotSuperuser => {
                ServiceError::Forbidden("must be a superuser".to_owned())
            }
        }
    }
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            middle_name: user.middle_name,
            email: user.email,
            phone: user.phone,
            superuser: user.superuser,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserId {
    pub id: i32,
}

macro_rules! impl_from_userlike {
    ($from_type: ty) => {
        impl From<$from_type> for UserId {
            fn from(source: $from_type) -> Self {
                Self { id: source.id }
            }
        }
    };
}

impl_from_userlike!(&User);
impl_from_userlike!(&UserInfo);

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    //issued at
    iat: i64,
    // expiry
    exp: i64,
    // user email
    user_id: i32,
}

// struct to get converted to token and back
impl Claims {
    fn with_user_id(user_id: i32) -> Self {
        Claims {
            user_id,
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::days(30)).timestamp(),
        }
    }
}

lazy_static! {
    static ref JWT_SECRET: Vec<u8> = {
        let secret = std::env::var("MONDRIAN_JWT_SECRET").unwrap();
        secret.into_bytes()
    };
}

impl UserId {
    pub fn get_token(&self) -> JWTResult<String> {
        let claims = Claims::with_user_id(self.id);

        encode(&Header::default(), &claims, &JWT_SECRET)
    }

    pub fn from_token(token: &str) -> JWTResult<Self> {
        decode::<Claims>(token, &JWT_SECRET, &Validation::default()).map(|data| data.claims.into())
    }
}

impl From<Claims> for UserId {
    fn from(claims: Claims) -> Self {
        Self { id: claims.user_id }
    }
}
