use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::model::{component::Component, rig::Rig};
use crate::model::rig::RigComponent;

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

#[derive(Serialize, Deserialize)]
pub struct CreateRigWithComponents {
    pub name: String,
    pub components: Vec<Uuid>
}

impl From<CreateRigWithComponents> for (Rig, Vec<RigComponent>){
    fn from (value: CreateRigWithComponents) -> Self {
        let CreateRigWithComponents { name, components } = value;

        let rig = Rig {
            id: Uuid::new_v4(),
            name,
        };

        let rig_components = components.into_iter()
            .map(|component_id| RigComponent {
                rig_id: rig.id,
                component_id
            })
            .collect();

        (rig, rig_components)
    }
}