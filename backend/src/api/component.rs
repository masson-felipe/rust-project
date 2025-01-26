use rocket::{fairing::AdHoc, get, routes, serde::json::Json};
use rocket_db_pools::{Connection, diesel::QueryResult};
use crate::{database::Db, model::component::Component};
use crate::repository::component;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Component Routes", |rocket| async {
        rocket.mount("/components", routes![list])
    })
}

#[get("/")]
pub async fn list(mut db: Connection<Db>) -> QueryResult<Json<Vec<Component>>> {
    Ok(Json(
        component::list_components(&mut db)
            .await?
        )
    )
}