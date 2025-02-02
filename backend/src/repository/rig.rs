use rocket_db_pools::{diesel::{GroupedBy, QueryDsl, SelectableHelper, BelongingToDsl, prelude::RunQueryDsl, QueryResult}, Connection};

use crate::{database::Db, dto::rig::RigWithComponents, model::{component::Component, rig::{Rig, RigComponent}}, schema::{components, rigs, rig_components}};
use crate::dto::rig::CreateRigWithComponents;
use crate::repository::component::list_components_by_ids;

pub async fn list_rigs(db: &mut Connection<Db>) -> QueryResult<Vec<RigWithComponents>> {
    let all_rigs = rigs::table
        .load::<Rig>(db)
        .await?;

    let rig_components = RigComponent::belonging_to(&all_rigs)
        .inner_join(components::table)
        .select((RigComponent::as_select(), Component::as_select()))
        .load(db)
        .await?;

    let rig_with_components: Vec<RigWithComponents> = rig_components
        .grouped_by(&all_rigs)
        .into_iter()
        .zip(all_rigs)
        .map(|(rcs, rig)| rig.with_components(
            rcs
                .into_iter()
                .map(|(_, component)| component)
                .collect()
        ))
        .collect();

    Ok(rig_with_components)
}

pub async fn create_rig_with_components(rig: CreateRigWithComponents, db: &mut Connection<Db>) -> QueryResult<RigWithComponents> {
    let (rig, rig_components) = rig.into();
    
    let rig = diesel::insert_into(rigs::table)
        .values(rig)
        .get_result::<Rig>(db)
        .await?;
    
    diesel::insert_into(rig_components::table)
        .values(&rig_components)
        .execute(db)
        .await?;
    
    let component_ids: Vec<uuid::Uuid> = rig_components
        .into_iter()
        .map(|rig_component| rig_component.component_id)
        .collect();
    
    let components = list_components_by_ids(&component_ids, db).await?;
    
    Ok(rig.with_components(components))
}

