// Diesel ins`t async first, so this import is using rocket, who is async first.
use rocket_db_pools::diesel::{Selectable, Identifiable, Insertable, QueryId, Queryable, AsChangeset};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::Slot;

// Querying (Queryable) and selecting (Selectable) are different concepts in Rust
// `Queryable` means you can look up for the table, and `Selectable` means that the data from the 
// table can be deserialized into the struct data.
#[derive(
    Serialize,
    Deserialize,
    QueryId,
    Queryable,
    Insertable,
    Identifiable,
    Selectable,
    AsChangeset
)]
#[diesel(table_name = crate::schema::components)]
pub struct Component {
    pub id: Uuid,
    pub manufacturer: String,
    pub model: String,
    pub slot: Slot,
    pub price: i32
}