pub mod api;
pub mod database;

use rocket::launch;
use rocket_db_pools::Database;
use crate::database::Db;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(crate::api::hello_world::stage())
}