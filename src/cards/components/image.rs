use bevy::{ecs::system::EntityCommands};

pub use crate::*;

#[derive(Reflect, Component, Default, Clone, serde::Serialize, serde::Deserialize)]
#[reflect(Component)]
pub struct CardImage;

#[derive(Default, Clone)]
pub struct ImageConstructor {
    pub texture_path: String,
}

impl CardComponent for CardImage{
    fn get_name(&self) -> String {
        "Image".to_string()
    }
}

impl Constructable for ImageConstructor {
    fn construct(&self, cmds: &mut EntityCommands, asset_server: &AssetServer, card_config: &CardConstructionConfig) {
        let texture = asset_server.load(&self.texture_path);
        let bun = SpriteBundle {
            texture: texture.clone(),
            sprite: Sprite {
                custom_size: Some(card_config.card_size),
                ..default()
            },
            ..default()
        };

        cmds.insert(CardImage.clone());
        cmds.insert(bun)
        .insert(Name::new("Card"))
        .insert(Interactable::default());
    }
}