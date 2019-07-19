use diesel::prelude::*;

pub use models::users::{NewUserEncrypted, NewUserPlain, User};

use crate::{apps::api::serializers::user::UserInfo, db::DbHandler, errors::ServiceError};

#[derive(Debug)]
pub struct NewUserRequest(pub NewUserPlain);

impl DbHandler {
    pub fn register(&self, msg: NewUserRequest) -> Result<UserInfo, ServiceError> {
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
