use diesel::prelude::*;

use models::users::User;

use crate::{apps::api::serializers::user::UserInfo, db::DbHandler, errors::ServiceError};

#[derive(Debug)]
pub struct UserRequest(pub i32);

impl DbHandler {
    pub fn get_user(&self, msg: UserRequest) -> Result<UserInfo, ServiceError> {
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
