use std::env;

use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware::Logger, App, HttpServer};
use chrono::Duration;
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use env_logger;
use r2d2;

mod apps;
mod db;
mod errors;

use self::apps::api;

type PgPool = r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let bind_to = env::var("MONDRIAN_BIND_TO").unwrap_or_else(|_| "127.0.0.1:3000".to_owned());

    let secret = std::env::var("MONDRIAN_SECRET_KEY").unwrap();
    let domain = std::env::var("MONDRIAN_DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let secure = std::env::var("MONDRIAN_HTTPS").unwrap().parse().unwrap();

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            // setup builtin logger to get nice logging for each request
            .wrap(Logger::default())
            // middleware for identity storage
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(secret.as_bytes())
                    .name("auth_token")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age_time(Duration::days(31))
                    .same_site(SameSite::Strict)
                    .secure(secure),
            ))
            .configure(|cfg| api::config_api(cfg, db_pool.clone()))
    })
    .bind(&bind_to)
    .unwrap_or_else(|e| panic!("Can not bind to '{}': {}", bind_to, e))
    .run()
    .await
}
