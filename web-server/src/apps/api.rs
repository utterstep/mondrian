use actix::Addr;
use actix_web::{
    http::Method,
    middleware::{
        identity::{CookieIdentityPolicy, IdentityService},
        Logger,
    },
    App,
};
use chrono::Duration;

use crate::db::DbExecutor;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

mod extractors;
mod handlers;
mod routes;
mod serializers;

// helper function to create and returns the app after mounting all routes/resources
pub fn create_app(db: Addr<DbExecutor>) -> App<AppState> {
    let secret = std::env::var("MONDRIAN_SECRET_KEY").unwrap();
    let domain = std::env::var("MONDRIAN_DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let secure = std::env::var("MONDRIAN_HTTPS").unwrap().parse().unwrap();

    App::with_state(AppState { db })
        .prefix("/api")
        // setup builtin logger to get nice logging for each request
        .middleware(Logger::default())
        // middleware for identity storage
        .middleware(IdentityService::new(
            CookieIdentityPolicy::new(secret.as_bytes())
                .name("auth_token")
                .path("/")
                .domain(domain.as_str())
                .max_age(Duration::days(31))
                .secure(secure),
        ))
        // router for registration
        .resource("/register/", |r| {
            r.method(Method::POST).with(routes::register::register_user)
        })
        // routes for authentication
        .resource("/auth/", |r| {
            r.method(Method::GET).with(routes::auth::get_me);
            r.method(Method::POST).with(routes::auth::login);
            r.method(Method::DELETE).with(routes::auth::logout);
        })
        // routes to user info
        .resource("/users/me/", |r| {
            r.method(Method::GET).with(routes::auth::get_me);
        })
        .resource("/users/me/id/", |r| {
            r.method(Method::GET).with(routes::auth::get_my_id);
        })
}
