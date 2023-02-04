use bevy::{ecs::system::EntityCommands, prelude::AssetServer};

use crate::CardConstructionConfig;

pub trait Constructable {
    fn construct(&self, cmds: &mut EntityCommands, asset_server: &AssetServer, card_config: &CardConstructionConfig);
}