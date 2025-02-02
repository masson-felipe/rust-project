use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::component::Component;
use crate::model::rig::Rig;

#[derive(Serialize, Deserialize)]
pub struct RigWithComponents {
    pub id: Uuid,
    pub name: String,
    pub components: Vec<Component>
}

impl Rig {
    pub fn with_components(self, components: Vec<Component>) -> RigWithComponents {
        let Rig { id, name} = self;
        RigWithComponents {
            id,
            name,
            components 
        }
    }
}