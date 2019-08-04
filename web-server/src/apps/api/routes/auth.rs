use actix_identity::Identity;
use actix_web::{
    error::BlockingError,
    web::{self, Data, Json},
    HttpResponse, Responder,
};

use futures::future::Future;

use crate::{
    apps::api::{
        handlers::auth::AuthData,
        serializers::user::{SuperuserInfo, UserId, UserInfo},
        AppData,
    },
    errors::ServiceError,
};

pub fn login(
    auth_data: Json<AuthData>,
    data: Data<AppData>,
    id: Identity,
) -> impl Future<Item = HttpResponse, Error = ServiceError> {
    let auth_data = auth_data.into_inner();

    web::block(move || data.db.login(auth_data)).then(move |res| match res {
        Ok(user_info) => {
            let user_id: UserId = (&user_info).into();
            let token = user_id
                .get_token()
                .map_err::<ServiceError, _>(|e| e.into())?;

            id.remember(token);

            Ok(HttpResponse::Ok().json(user_info))
        }
        Err(BlockingError::Error(err)) => Err(err),
        Err(BlockingError::Canceled) => Err(ServiceError::InternalServerError),
    })
}

pub fn logout(id: Identity) -> impl Responder {
    id.forget();

    HttpResponse::NoContent()
}

pub fn get_me(user_info: UserInfo) -> impl Responder {
    HttpResponse::Ok().json(user_info)
}

pub fn get_my_id(user_id: UserId) -> impl Responder {
    HttpResponse::Ok().json(user_id)
}

pub fn is_superuser(_superuser: SuperuserInfo) -> impl Responder {
    HttpResponse::NoContent()
}
