use rocket::{fairing::AdHoc, get, post, routes, serde::json::Json};
use rocket_db_pools::{Connection, diesel::QueryResult};
use crate::{database::Db};
use crate::dto::rig::{CreateRigWithComponents, RigWithComponents};
use crate::repository::rig;
use crate::repository::rig::create_rig_with_components;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Rig Routes", |rocket| async {
        rocket.mount("/rigs", routes![list, create])
    })
}

#[get("/")]
pub async fn list(mut db: Connection<Db>) -> QueryResult<Json<Vec<RigWithComponents>>> {
    Ok(Json(
        rig::list_rigs(&mut db)
            .await?
    ))
}

#[post("/", data = "<rig>")]
async fn create(rig: Json<CreateRigWithComponents>, mut db: Connection<Db>) -> QueryResult<Json<RigWithComponents>> {
    Ok(Json(
        create_rig_with_components(rig.0, &mut db).await?
    ))
}
