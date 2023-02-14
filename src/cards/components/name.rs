use bevy::{ecs::system::EntityCommands};

pub use crate::*;

#[derive(Default, Clone)]
pub struct NameConstructor {
    pub name: String,
}

#[derive(Reflect, Component, Default, Clone, serde::Serialize, serde::Deserialize)]
#[reflect(Component)]
pub struct CardName{
    pub name: String,
}

impl CardComponent for CardName{
    fn get_name(&self) -> String {
        "Name".to_string()
    }
}

impl Constructable for NameConstructor {
    fn construct(&self, cmds: &mut EntityCommands, _: &AssetServer, _: &CardConstructionConfig) {
        cmds.insert(CardName{ name: self.name.clone() });
        cmds.insert(Name::new(self.name.clone()));
    }
}