use std::convert::TryInto;

use actix_identity::Identity;
use actix_web::{
    dev::Payload,
    error::BlockingError,
    web::{self, Data},
    FromRequest, HttpRequest,
};

use futures::{Future, IntoFuture};

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
    type Future = Result<Self, Self::Error>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let id = Identity::from_request(req, payload);

        match id {
            Ok(id) => match id.identity() {
                Some(identity) => Ok(UserId::from_token(&identity)?),
                None => Err(ServiceError::Unauthorized),
            },
            Err(_) => Err(ServiceError::InternalServerError),
        }
    }
}

impl FromRequest for UserInfo {
    type Config = ();
    type Error = ServiceError;
    type Future = Box<Future<Item = Self, Error = Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let data = Data::from_request(req, payload).map_err(|_| ServiceError::InternalServerError);
        let user_id = UserId::from_request(req, payload).into_future();

        Box::new(
            user_id
                .join(data)
                .and_then(move |(user_id, data): (UserId, Data<AppData>)| {
                    web::block(move || data.db.get_user(UserRequest(user_id.id))).map_err(|err| {
                        match err {
                            BlockingError::Error(err) => err,
                            BlockingError::Canceled => ServiceError::InternalServerError,
                        }
                    })
                }),
        )
    }
}

impl FromRequest for SuperuserInfo {
    type Config = ();
    type Error = ServiceError;
    type Future = Box<Future<Item = Self, Error = Self::Error>>;

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
