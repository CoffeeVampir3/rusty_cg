use bevy::{ecs::system::EntityCommands, text::Text2dBounds};

pub use crate::*;

#[derive(Default, Clone)]
pub struct DescriptionConstructor {
    pub desc: String,
}

#[derive(Reflect, Component, Default, Clone)]
#[reflect(Component)]
pub struct CardDescription {
    pub desc: String,
}

impl Constructable for DescriptionConstructor {
    fn construct(&self, cmds: &mut EntityCommands, _: &AssetServer, card_config: &CardConstructionConfig) {
        let a = Text2dBundle {
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
        cmds.insert(CardDescription{desc:self.desc.clone()});
        cmds.with_children(|x| {
            x.spawn(a).insert(Name::new("Description"));
        });
    }
}
