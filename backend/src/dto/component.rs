use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{model::component::Component, types::Slot};

#[derive(Serialize, Deserialize)]
pub struct CreateComponentData {
    pub manufacturer: String,
    pub model: String,
    pub slot: Slot,
    pub price: i32
}

impl From<CreateComponentData> for Component {
    fn from(value: CreateComponentData) -> Self {
        let id = Uuid::new_v4();
        let CreateComponentData {manufacturer, model, slot, price} = value;
        Self {
            id,
            manufacturer,
            model,
            slot,
            price 
        }
    }
}