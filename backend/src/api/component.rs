use rocket::{fairing::AdHoc, get, post, routes, serde::json::Json};
use rocket_db_pools::{Connection, diesel::QueryResult};
use crate::{database::Db, model::component::Component};
use crate::dto::component::CreateComponentData;
use crate::repository::component;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Component Routes", |rocket| async {
        rocket.mount("/components", routes![list, create])
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

#[post("/", data = "<component>")]
pub async fn create(component: Json<CreateComponentData>, mut db: Connection<Db>) -> QueryResult<Json<Component>> {
    Ok(Json(
        component::insert_component(component.0.into(), &mut db)
            .await?
    ))
}