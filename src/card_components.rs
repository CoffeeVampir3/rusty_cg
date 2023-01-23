pub use crate::*;
use bevy::{text::Text2dBounds, ecs::system::EntityCommands};

pub trait Constructable {
    fn construct(&self, cmds: &mut EntityCommands, card_config: &CardConstructionConfig);
}

#[derive(Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct Card;

#[derive(Default)]
pub struct CardConstructor {
    pub texture: Handle<Image>,
}

impl Constructable for CardConstructor {
    fn construct(&self, cmds: &mut EntityCommands, card_config: &CardConstructionConfig) {
        let bun = SpriteBundle {
            texture: self.texture.clone(),
            sprite: Sprite {
                custom_size: Some(card_config.card_size),
                ..default()
            },
            ..default()
        };

        cmds.insert(Card.clone());
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

#[derive(Default)]
pub struct CardDescriptionConstructor {
    pub description: CardDescription,
}

#[derive(Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct CardDescription {
    pub desc: String,
}

impl Constructable for CardDescriptionConstructor {
    fn construct(&self, cmds: &mut EntityCommands, card_config: &CardConstructionConfig) {
        let a = Text2dBundle {
            text: Text::from_section(self.description.desc.clone(), card_config.text_style.clone())
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
        cmds.insert(self.description.clone());
        cmds.with_children(|x| {
            x.spawn(a).insert(Name::new("Description"));
        });
    }
}

#[derive(Default)]
pub struct CardCostConstructor {
    pub cost: CardCost,
}

#[derive(Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct CardCost {
    pub cost: u32,
}

impl Constructable for CardCostConstructor {
    fn construct(&self, cmds: &mut EntityCommands, card_config: &CardConstructionConfig) {
        let a = Text2dBundle {
            text: Text::from_section(self.cost.cost.to_string(), card_config.text_style.clone())
                .with_alignment(card_config.text_alignment),
            transform: Transform::from_xyz(
                card_config.card_width / 4.0,
                card_config.card_height / 3.0,
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
        cmds.insert(self.cost.clone());
        cmds.with_children(|x| {
            x.spawn(a).insert(Name::new("Cost"));
        });
    }
}