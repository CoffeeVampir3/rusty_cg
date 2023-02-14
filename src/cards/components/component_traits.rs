use bevy::{ecs::system::EntityCommands, prelude::AssetServer};

use crate::CardConstructionConfig;

#[bevy_trait_query::queryable]
pub trait CardComponent: {
    fn get_name(&self) -> String;
}

pub trait Constructable {
    fn construct(&self, cmds: &mut EntityCommands, asset_server: &AssetServer, card_config: &CardConstructionConfig);
}