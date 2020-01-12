use std::{convert::TryInto, future::Future};

use actix_identity::Identity;
use actix_web::{
    dev::Payload,
    error::BlockingError,
    web::{self, Data},
    FromRequest, HttpRequest,
};

use futures::future::{ok, err, ready, Ready, TryFutureExt, FutureObj};

use crate::{
    apps::api::{
        handlers::get_user::UserRequest,
        serializers::user::{SuperuserConversionError, SuperuserInfo, UserId, UserInfo},
        AppData,
    },
    errors::ServiceError,
};

impl FromRequest for UserId {
    type Config = ();
    type Error = ServiceError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let id = req.app_data::<Identity>();

        match id {
            Some(identity) => {
                match identity.identity() {
                    Some(identity) => ready(UserId::from_token(&identity).map_err(From::from)),
                    None => err(ServiceError::Unauthorized),
                }
            },
            None => err(ServiceError::InternalServerError),
        }
    }
}

impl FromRequest for UserInfo {
    type Config = ();
    type Error = ServiceError;
    type Future = Box<Future<Output = Result<Self, Self::Error>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        macro_rules! unwrap_or {
            ($opt: expr, $error: expr) => {
                match $opt {
                    Some(value) => value,
                    None => return Box::new(err($error))
                }
            };
        }

        let data = unwrap_or!(req.app_data::<AppData>(), ServiceError::InternalServerError);
        let user_id = unwrap_or!(req.app_data::<UserId>(), ServiceError::Unauthorized);

        Box::new(web::block(move || data.db.get_user(UserRequest(user_id.id))).map_err(|err| {
            match err {
                BlockingError::Error(err) => err,
                BlockingError::Canceled => ServiceError::InternalServerError,
            }
        }))
    }
}

impl FromRequest for SuperuserInfo {
    type Config = ();
    type Error = ServiceError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        Box::new(
            UserInfo::from_request(req, payload).and_then(move |user_info| {
                user_info
                    .try_into()
                    .map_err(|e: SuperuserConversionError| e.into())
            }),
        )
    }
}
