use rocket::FromFormField;
use serde::{Deserialize, Serialize};

#[derive(diesel_derive_enum::DbEnum, Clone, Copy, Debug, Serialize, Deserialize, FromFormField)]
#[ExistingTypePath = "crate::schema::sql_types::Slot"]
pub enum Slot {
    Cpu,
    Gpu,
    Memory,
    Storage
}