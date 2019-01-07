use actix_web::{middleware::identity::RequestIdentity, Error, FromRequest, HttpRequest};

use futures::{Future, Poll};

// use models::users::User;

use crate::{
    apps::api::{
        handlers::get_user::UserRequest,
        serializers::user::{UserId, UserInfo},
        AppState,
    },
    errors::ServiceError,
};

impl<S> FromRequest<S> for UserId {
    type Config = ();
    type Result = Result<Self, ServiceError>;

    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        match req.identity() {
            Some(identity) => Ok(UserId::from_token(&identity)?),
            None => Err(ServiceError::Unauthorized),
        }
    }
}

impl FromRequest<AppState> for UserInfo {
    type Config = ();
    type Result = Box<Future<Item = Self, Error = Error>>;

    fn from_request(req: &HttpRequest<AppState>, _: &Self::Config) -> Self::Result {
        Box::new(UserInfoResult::from_req(req))
    }
}

struct UserInfoResult {
    fut: Option<Box<Future<Item = UserInfo, Error = Error>>>,
    err: Option<Error>,
}

impl UserInfoResult {
    fn from_err(err: ServiceError) -> Self {
        Self {
            err: Some(err.into()),
            fut: None,
        }
    }

    fn from_req(req: &HttpRequest<AppState>) -> Self {
        let user_id = match req.identity() {
            Some(identity) => UserId::from_token(&identity),
            None => {
                return Self::from_err(ServiceError::Unauthorized);
            }
        };

        let user_id = match user_id {
            Ok(user_id) => user_id.id,
            Err(jwt_error) => {
                return Self::from_err(jwt_error.into());
            }
        };

        Self {
            err: None,
            fut: Some(Box::new(
                req.state()
                    .db
                    .send(UserRequest(user_id))
                    .from_err::<Error>()
                    .and_then(|db_response| match db_response {
                        Ok(user) => Ok(user),
                        Err(db_error) => Err(db_error.into()),
                    }),
            )),
        }
    }
}

impl Future for UserInfoResult {
    type Item = UserInfo;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if let Some(err) = self.err.take() {
            return Err(err);
        }

        if let Some(ref mut fut) = self.fut {
            return fut.poll();
        }

        // struct was instantiated empty
        Err(ServiceError::InternalServerError.into())
    }
}
