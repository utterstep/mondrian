use actix_identity::Identity;
use actix_web::{
    error::BlockingError,
    web::{self, Data, Json},
    HttpResponse,
};

use crate::{
    apps::api::{
        handlers::register::{NewUserPlain, NewUserRequest},
        serializers::user::UserId,
        AppData,
    },
    errors::ServiceError,
};

pub async fn register_user(
    user_data: Json<NewUserPlain>,
    data: Data<AppData>,
    id: Identity,
) -> Result<HttpResponse, ServiceError> {
    let new_user = NewUserRequest(user_data.into_inner());

    let res = web::block(move || data.db.register(new_user)).await;

    match res {
        Ok(user) => {
            let user_id: UserId = (&user).into();
            let token = user_id
                .get_token()
                .map_err::<ServiceError, _>(|e| e.into())?;

            id.remember(token);

            Ok(HttpResponse::Created().json(user))
        }
        Err(BlockingError::Error(err)) => Err(err),
        Err(BlockingError::Canceled) => Err(ServiceError::InternalServerError),
    }
}
