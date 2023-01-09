use std::any::Any;

use bevy::text::Text2dBounds;

pub use crate::*;
pub struct CardConstructionKitPlugin;

impl Plugin for CardConstructionKitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_construction_config)
            .add_startup_system(test_initialize)
            .add_system(run_construction);
    }
}

#[derive(Resource)]
struct CardConstructionConfig {
    pub card_width: f32,
    pub card_height: f32,
    pub card_size: Vec2,
    pub text_alignment: TextAlignment,
    pub magic_number: f32,
    pub text_style: TextStyle,
}

fn initialize_construction_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.insert_resource(CardConstructionConfig {
        card_width: 200.0,
        card_height: 300.0,
        card_size: Vec2 { x: 200.0, y: 300.0 },
        text_alignment: TextAlignment::CENTER,
        magic_number: 0.5 / 524288.0,
        text_style: TextStyle {
            font_size: 24.0,
            font: font,
            color: Color::BLACK,
        },
    });
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Card {
    pub texture: Handle<Image>,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CardDescription {
    pub desc: String,
}

fn test_initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Card {
            texture: asset_server.load("test/whatever.png"),
        })
        .insert(CardDescription {
            desc: "Hello World".to_string(),
        });

    commands
        .spawn(Card {
            texture: asset_server.load("test/whatever.png"),
        })
        .insert(CardDescription {
            desc: "Hello World2".to_string(),
        });

    commands
        .spawn(Card {
            texture: asset_server.load("test/whatever.png"),
        })
        .insert(CardDescription {
            desc: "Hello World3".to_string(),
        });

    commands
        .spawn(Card {
            texture: asset_server.load("test/whatever.png"),
        })
        .insert(CardDescription {
            desc: "Hello World4".to_string(),
        });
}

pub trait Summary {
    fn summarize(&self) -> String;
}

trait Construct {
    type ConstructedType: Bundle;
    fn construct(&self, card_config: &CardConstructionConfig) -> Self::ConstructedType;
}

impl Construct for CardDescription {
    type ConstructedType = Text2dBundle;
    fn construct(&self, card_config: &CardConstructionConfig) -> Text2dBundle {
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

fn run_construction(
    mut commands: Commands,
    mut top_layer: ResMut<TopLayer>,
    unconstructed_cards: Query<(Entity, &Card, Option<&CardDescription>)>,
    card_config: Res<CardConstructionConfig>,
) {
    for (ent, card, desc) in &unconstructed_cards {
        let z_height = top_layer.top();

        let sprite_bundle = SpriteBundle {
            texture: card.texture.clone(),
            transform: Transform::from_xyz(0.0, 0.0, z_height),
            sprite: Sprite {
                custom_size: Some(card_config.card_size),
                ..default()
            },
            ..default()
        };

        commands.entity(ent).despawn_recursive();

        let mut spawned = commands.spawn(sprite_bundle);
        spawned
            .insert(Name::new("test_sprite"))
            .insert(Collider::cuboid(
                card_config.card_width / 2.0,
                card_config.card_height / 2.0,
            ))
            .insert(Sensor);

        if desc.is_some() {
            let desc = desc.unwrap();
            let comp = desc.construct(card_config.as_ref());

            spawned.with_children(|x| {
                x.spawn(comp).insert(Name::new("card description"));
            });
        }

        println!("Constructed a card {ent:?}");
    }
}
