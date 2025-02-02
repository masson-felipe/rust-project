use rocket_db_pools::{diesel::{GroupedBy, QueryDsl, BelongingToDsl, SelectableHelper, prelude::RunQueryDsl, QueryResult}, Connection};

use crate::{database::Db, dto::rig::RigWithComponents, model::rig::{Rig, RigComponent}, schema::{rigs, rig_components}};
use crate::model::component::Component;

pub async fn list_rigs(db: &mut Connection<Db>) -> QueryResult<Vec<RigWithComponents>> {
    let all_rigs = rigs::table
        .load::<Rig>(db)
        .await?;

    let rig_components = RigComponent::belonging_to(&all_rigs)
        .inner_join(components::table)
        .select(RigComponent::as_select(), Component::as_select())
        .load(db).await?;

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