use actix_web::{
    middleware::identity::RequestIdentity, AsyncResponder, Error, HttpRequest, HttpResponse, Json,
    Responder, ResponseError,
};
use futures::future::Future;

use crate::{
    apps::api::{
        handlers::register::{NewUserPlain, NewUserRequest},
        serializers::user::UserId,
        AppState,
    },
    errors::ServiceError,
};

pub fn register_user(
    (user_data, req): (Json<NewUserPlain>, HttpRequest<AppState>),
) -> impl Responder {
    let db_message = NewUserRequest(user_data.into_inner());

    req.state()
        .db
        .send(db_message)
        .from_err::<Error>()
        .and_then(move |db_response| match db_response {
            Ok(user) => {
                let user_id: UserId = (&user).into();
                let token = user_id
                    .get_token()
                    .map_err::<ServiceError, _>(|e| e.into())?;

                req.remember(token);

                Ok(HttpResponse::Created().json(user))
            }
            Err(service_error) => Ok(service_error.error_response()),
        })
        .responder()
}
