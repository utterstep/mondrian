use std::env;

use actix::{Addr, SyncArbiter};
use actix_web::server;
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use env_logger;
use r2d2;

mod apps;
mod db;
mod errors;

use self::apps::api;
use self::db::DbExecutor;

fn main() {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let sys = actix::System::new("mondrian");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let address: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    server::new(move || api::create_app(address.clone()))
        .bind("127.0.0.1:3000")
        .expect("Can not bind to '127.0.0.1:3000'")
        .start();

    sys.run();
}
