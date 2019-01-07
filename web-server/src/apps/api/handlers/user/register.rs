use actix::{Handler, Message};
use diesel::prelude::*;

pub use models::users::{NewUserEncrypted, NewUserPlain, User};

use crate::{apps::api::serializers::user::UserInfo, db::DbExecutor, errors::ServiceError};

#[derive(Debug)]
pub struct NewUserRequest(pub NewUserPlain);

impl Message for NewUserRequest {
    type Result = Result<UserInfo, ServiceError>;
}

impl Handler<NewUserRequest> for DbExecutor {
    type Result = Result<UserInfo, ServiceError>;

    fn handle(&mut self, msg: NewUserRequest, _: &mut Self::Context) -> Self::Result {
        use models::schema::users::dsl::users;

        let conn = &self
            .0
            .get()
            .map_err(|_| ServiceError::InternalServerError)?;
        let user_to_insert = NewUserEncrypted::from_plain(msg.0)?;

        let inserted_user: User = diesel::insert_into(users)
            .values(&user_to_insert)
            .get_result(conn)?;

        Ok(inserted_user.into())
    }
}
