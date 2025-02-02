use rocket_db_pools::{diesel::{prelude::RunQueryDsl, QueryResult, QueryDsl, ExpressionMethods}, Connection};
use uuid::Uuid;
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

pub async fn list_components_by_ids(ids: &[Uuid], db: &mut Connection<Db>) -> QueryResult<Vec<Component>> {
    Ok(components::table
        .filter(components::id.eq_any(ids))
        .load::<Component>(db)
        .await?
    )
}