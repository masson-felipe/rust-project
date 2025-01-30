use rocket_db_pools::{diesel::{prelude::RunQueryDsl, QueryResult}, Connection};
use crate::{database::Db, model::component::Component, schema::components};

pub async fn list_components(db: &mut Connection<Db>) -> QueryResult<Vec<Component>> {
    Ok(components::table
        .load::<Component>(db)
        .await?
    )
}

pub async fn insert_component(component: Component, db: &mut Connection<Db>) -> QueryResult<Component> {
    Ok(
        diesel::insert_into(components::table)
            .values(component)
            .get_result::<Component>(db)
            .await?
    )
}