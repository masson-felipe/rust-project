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