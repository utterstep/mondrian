use actix::{Handler, Message};
use diesel::prelude::*;

use models::users::User;

use crate::{apps::api::serializers::user::UserInfo, db::DbExecutor, errors::ServiceError};

#[derive(Debug)]
pub struct UserRequest(pub i32);

impl Message for UserRequest {
    type Result = Result<UserInfo, ServiceError>;
}

impl Handler<UserRequest> for DbExecutor {
    type Result = Result<UserInfo, ServiceError>;

    fn handle(&mut self, msg: UserRequest, _: &mut Self::Context) -> Self::Result {
        use models::schema::users::dsl::{id, users};

        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceError::InternalServerError)?;

        let user = users
            .filter(id.eq(&msg.0))
            .first::<User>(conn)
            .optional()?
            .ok_or_else(|| ServiceError::BadRequest(format!("no user with id {}", &msg.0)))?;

        Ok(user.into())
    }
}
