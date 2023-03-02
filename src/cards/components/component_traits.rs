use bevy::{ecs::system::EntityCommands, prelude::AssetServer};
use dyn_clone::DynClone;

use crate::CardConstructionConfig;

#[bevy_trait_query::queryable]
pub trait CardComponent {
    fn get_name(&self) -> String;
}

pub trait Constructable: DynClone {
    fn construct(&self, cmds: &mut EntityCommands, asset_server: &AssetServer, card_config: &CardConstructionConfig);
}

dyn_clone::clone_trait_object!(Constructable);

pub trait Duplicate {
    fn duplicate(&self) -> Self;
}

impl Duplicate for Vec::<Box<dyn Constructable>> {
    fn duplicate(&self) -> Self {
        let mut clone = Vec::<Box<dyn Constructable>>::new();
        for k in self {
            clone.push(k.clone())
        }
        clone
    }
}