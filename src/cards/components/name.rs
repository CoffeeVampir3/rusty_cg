use bevy::{ecs::system::EntityCommands};

pub use crate::*;

#[derive(Default, Clone)]
pub struct NameConstructor {
    pub name: String,
}

#[derive(Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct CardName{
    pub name: String,
}

impl Constructable for NameConstructor {
    fn construct(&self, cmds: &mut EntityCommands, _: &AssetServer, _: &CardConstructionConfig) {
        cmds.insert(Name::new(self.name.clone()));
    }
}