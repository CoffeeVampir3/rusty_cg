use bevy::ecs::system::EntityCommands;

pub use crate::*;

#[derive(Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct CardImage;

#[derive(Default, Clone)]
pub struct ImageConstructor {
    pub texture_path: String,
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
        .insert(Collider::cuboid(
            card_config.card_width / 2.0,
            card_config.card_height / 2.0,
        ))
        .insert(Sensor)
        .insert(Interactable::default());
    }
}