use diesel::prelude::*;
use serde_derive::Deserialize;

use mondrian_models::users::User;

use crate::{apps::api::serializers::user::UserInfo, db::DbHandler, errors::ServiceError};

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
}

impl DbHandler {
    pub fn login(&self, msg: AuthData) -> Result<UserInfo, ServiceError> {
        use mondrian_models::schema::users::dsl::{email, phone, users};

        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceError::InternalServerError)?;

        let query = users.into_boxed();

        let query = match (msg.email, msg.phone) {
            (Some(e), None) => query.filter(email.eq(e)),
            (None, Some(p)) => query.filter(phone.eq(p)),
            (Some(e), Some(p)) => query.filter(phone.eq(p).and(email.eq(e))),
            _ => return Err(ServiceError::Unauthorized),
        };

        let user = query.first::<User>(conn).optional()?;

        if let Some(user) = user {
            if !user.superuser {
                return Ok(user.into());
            }

            if let Some(password) = msg.password {
                match user.check_password(&password) {
                    Ok(valid) => {
                        if valid {
                            return Ok(user.into());
                        } else {
                            return Err(ServiceError::Unauthorized);
                        }
                    }
                    _ => return Err(ServiceError::Unauthorized),
                }
            }

            return Err(ServiceError::Forbidden(
                "Password should be supplied for superuser accounts".into(),
            ));
        }

        Err(ServiceError::Unauthorized)
    }
}
