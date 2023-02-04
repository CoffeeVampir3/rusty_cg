use bevy::{prelude::*, ecs::system::EntityCommands, text::Text2dBounds};

use crate::{Constructable, CardConstructionConfig};

#[derive(Default, Clone)]
pub struct CostConstructor {
    pub cost: CardCost,
}

#[derive(Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct CardCost {
    pub cost: u32,
}

impl Constructable for CostConstructor {
    fn construct(&self, cmds: &mut EntityCommands, _: &AssetServer, card_config: &CardConstructionConfig) {
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