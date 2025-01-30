pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "slot"))]
    pub struct Slot; // Opaque Type
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Slot;

    components (id) {
        id -> Uuid,
        manufacturer -> VarChar,
        model -> VarChar,
        slot -> Slot,
        price -> Int4
    }
}

diesel::table! {
    rigs (id) {
        id -> Uuid,
        name -> VarChar
    }
}

diesel::table! {
    rig_components (rig_id, component_id) {
        rig_id -> Uuid,
        component_id -> Uuid
    }
}

diesel::joinable!(rig_components -> rigs (rig_id));
diesel::joinable!(rig_components -> components (component_id));

diesel::allow_tables_to_appear_in_same_query!(rigs, components, rig_components);