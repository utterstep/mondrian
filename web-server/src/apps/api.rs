use actix_web::web;

use crate::{db::DbHandler, PgPool};

pub struct AppData {
    pub db: DbHandler,
}

mod extractors;
mod handlers;
mod routes;
mod serializers;

// helper function to create and returns the app after mounting all routes/resources
pub fn config_api(cfg: &mut web::ServiceConfig, db: PgPool) {
    cfg.data(AppData { db: DbHandler(db) })
        // router for registration
        .route(
            "/api/register/",
            web::post().to_async(routes::register::register_user),
        )
        // routes for authentication
        .service(
            web::resource("/api/auth/")
                .route(web::get().to(routes::auth::get_me))
                .route(web::post().to_async(routes::auth::login))
                .route(web::delete().to(routes::auth::logout)),
        )
        .route("/api/users/me/", web::get().to(routes::auth::get_me))
        .route("/api/users/me/id/", web::get().to(routes::auth::get_my_id));
}
