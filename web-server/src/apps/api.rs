use actix_web::web;

use crate::{db::DbHandler, PgPool};

pub struct AppData {
    pub db: DbHandler,
}

mod extractors;
mod handlers;
mod routes;
mod serializers;

// helper function mounting all routes/resources
pub fn config_api(cfg: &mut web::ServiceConfig, db: PgPool) {
    cfg.data(AppData { db: DbHandler(db) }).service(
        web::scope("/api")
            // router for registration
            .route(
                "/register/",
                web::post().to_async(routes::register::register_user),
            )
            // routes for authentication
            .service(
                web::resource("/auth/")
                    .route(web::get().to(routes::auth::get_me))
                    .route(web::post().to_async(routes::auth::login))
                    .route(web::delete().to(routes::auth::logout)),
            )
            // current user information
            .route("/users/me/", web::get().to(routes::auth::get_me))
            .route("/users/me/id/", web::get().to(routes::auth::get_my_id)),
    );
}
