use actix_web::{
    middleware::identity::RequestIdentity, AsyncResponder, Error, HttpRequest, HttpResponse, Json,
    Responder, ResponseError,
};
use futures::future::Future;

use crate::{
    apps::api::{
        handlers::auth::AuthData,
        serializers::user::{UserId, UserInfo},
        AppState,
    },
    errors::ServiceError,
};

pub fn login((auth_data, req): (Json<AuthData>, HttpRequest<AppState>)) -> impl Responder {
    let auth_data = auth_data.into_inner();

    req.state()
        .db
        .send(auth_data)
        .from_err::<Error>()
        .and_then(move |db_response| match db_response {
            Ok(user_id) => {
                let token = user_id
                    .get_token()
                    .map_err::<ServiceError, _>(|e| e.into())?;

                req.remember(token);

                Ok(HttpResponse::Ok().json(user_id))
            }
            Err(service_error) => Ok(service_error.error_response()),
        })
        .responder()
}

// for actix FromRequest trait
#[allow(clippy::needless_pass_by_value)]
pub fn logout(req: HttpRequest<AppState>) -> impl Responder {
    req.forget();

    HttpResponse::NoContent()
}

pub fn get_me(user_info: UserInfo) -> impl Responder {
    HttpResponse::Ok().json(user_info)
}

pub fn get_my_id(user_id: UserId) -> impl Responder {
    HttpResponse::Ok().json(user_id)
}
