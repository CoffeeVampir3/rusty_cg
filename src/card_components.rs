use bevy::text::Text2dBounds;
pub use crate::*;

pub trait Construct {
    type ConstructedType: Bundle;
    fn construct(&self, card_config: &CardConstructionConfig) -> Self::ConstructedType;
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Card {
    pub texture: Handle<Image>,
}

impl Construct for Card {
    type ConstructedType = SpriteBundle;
    fn construct(&self, card_config: &CardConstructionConfig) -> Self::ConstructedType {
        let sprite_bundle = SpriteBundle {
            texture: self.texture.clone(),
            sprite: Sprite {
                custom_size: Some(card_config.card_size),
                ..default()
            },
            ..default()
        };
        return sprite_bundle;
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CardDescription {
    pub desc: String,
}

impl Construct for CardDescription {
    type ConstructedType = Text2dBundle;
    fn construct(&self, card_config: &CardConstructionConfig) -> Self::ConstructedType {
        let card_desc = Text2dBundle {
            text: Text::from_section(self.desc.clone(), card_config.text_style.clone())
                .with_alignment(card_config.text_alignment),
            transform: Transform::from_xyz(
                0.0,
                -card_config.card_height / 3.0,
                card_config.magic_number,
            ),
            text_2d_bounds: Text2dBounds {
                size: Vec2 {
                    x: card_config.card_width,
                    y: card_config.card_height,
                },
            },
            ..default()
        };
        card_desc
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CardManaCost {
    pub cost: u32,
}

impl Construct for CardManaCost {
    type ConstructedType = Text2dBundle;
    fn construct(&self, card_config: &CardConstructionConfig) -> Self::ConstructedType {
        let card_desc = Text2dBundle {
            text: Text::from_section(self.cost.to_string(), card_config.text_style.clone())
                .with_alignment(card_config.text_alignment),
            transform: Transform::from_xyz(
                0.0,
                -card_config.card_height / 3.0,
                card_config.magic_number,
            ),
            text_2d_bounds: Text2dBounds {
                size: Vec2 {
                    x: card_config.card_width,
                    y: card_config.card_height,
                },
            },
            ..default()
        };
        card_desc
    }
}