use rocket_db_pools::diesel::{Selectable, AsChangeset, Associations, Identifiable, Insertable, QueryId, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::component::Component;

#[derive(
    Serialize,
    Deserialize,
    QueryId,
    Queryable,
    Insertable,
    AsChangeset,
    Identifiable,
    Selectable
)]
#[diesel(table_name = crate::schema::rigs)]
pub struct Rig {
    pub id: Uuid,
    pub name: String
}

#[derive(
    Serialize,
    Deserialize,
    QueryId,
    Queryable,
    Insertable,
    Identifiable,
    Associations,
    Selectable
)]
#[diesel(table_name = crate::schema::rig_components)]
/* Necessário para que o parâmetro 'Identifiable' funcione, visto que ele espera primary key
padrão como 'id'. Esta é uma tabela pivot. */
#[diesel(primary_key(rig_id, component_id))]
#[diesel(belongs_to(Rig))]
#[diesel(belongs_to(Component))]
pub struct RigComponent {
    pub rig_id: Uuid,
    pub component_id: Uuid
}