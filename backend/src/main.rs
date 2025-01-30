pub mod api;
pub mod database;
pub mod model;
pub mod types;
pub mod schema;
pub mod repository;
mod dto;

use rocket::launch;
use rocket_db_pools::Database;
use database::Db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(crate::api::component::stage())
        .attach(crate::api::hello_world::stage())
}