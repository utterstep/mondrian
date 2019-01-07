use actix::{Handler, Message};
use diesel::prelude::*;
use serde_derive::Deserialize;

use models::users::User;

use crate::{apps::api::serializers::user::UserId, db::DbExecutor, errors::ServiceError};

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
}

impl Message for AuthData {
    type Result = Result<UserId, ServiceError>;
}

impl Handler<AuthData> for DbExecutor {
    type Result = Result<UserId, ServiceError>;

    fn handle(&mut self, msg: AuthData, _: &mut Self::Context) -> Self::Result {
        use models::schema::users::dsl::{email, phone, users};

        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceError::InternalServerError)?;
        let incomplete_data_error = Err(ServiceError::BadRequest(
            "Password should be supplied for superuser accounts".into(),
        ));
        let mismatch_error = Err(ServiceError::Unauthorized);

        let query = users.into_boxed();

        let query = match (msg.email, msg.phone) {
            (Some(e), None) => query.filter(email.eq(e)),
            (None, Some(p)) => query.filter(phone.eq(p)),
            (Some(e), Some(p)) => query.filter(phone.eq(p).and(email.eq(e))),
            _ => return Err(ServiceError::Unauthorized),
        };

        let user = query.first::<User>(conn).optional()?;

        if let Some(user) = &user {
            if !user.superuser {
                return Ok(user.into());
            }

            if let Some(password) = msg.password {
                match user.check_password(&password) {
                    Ok(valid) => {
                        if valid {
                            return Ok(user.into());
                        } else {
                            return mismatch_error;
                        }
                    }
                    _ => return mismatch_error,
                }
            }

            return incomplete_data_error;
        }

        mismatch_error
    }
}
