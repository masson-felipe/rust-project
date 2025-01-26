use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::Slot;

#[derive(Serialize, Deserialize)]
pub struct Component {
    pub id: Uuid,
    pub manufacturer: String,
    pub model: String,
    pub slot: Slot,
    pub price: i32
}