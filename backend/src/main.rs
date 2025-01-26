pub mod api;
pub mod database;
pub mod model;
mod types;
mod schema;

use rocket::launch;
use rocket_db_pools::Database;
use database::Db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(crate::api::hello_world::stage())
}