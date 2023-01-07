mod spritedragdrop;
mod cgsys;
use bevy::text::Text2dBounds;
pub use cgsys::*;
pub use spritedragdrop::*;

use bevy::prelude::*;
use bevy_rapier2d::render::*;
use bevy_rapier2d::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui_rapier::*;

use rand::Rng;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(InspectableRapierPlugin)
    .add_plugin(WorldInspectorPlugin)
    .add_plugin(CGCorePlugin)
    .add_plugin(CGSys)
    .run();
}

pub struct CGCorePlugin;

impl Plugin for CGCorePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let card_width = 200.0;
    let card_height = 300.0;
    let card_size = Vec2 {x:card_width, y:card_height};
    let text_alignment = TextAlignment::CENTER;
    let text_style = TextStyle {
        font_size: 20.0,
        font,
        color: Color::WHITE,
    };

    for x in (-2..=2).map(|x| x as f32 * card_width/2.0) {
        let sprite_bundle = SpriteBundle {
            texture: asset_server.load("test/whatever.png"),
            transform: Transform::from_xyz(x, 0.0, ((x / card_width) + 8.0)/524288.0),
            sprite: Sprite {
                color: Color::Rgba { 
                    red: rand::thread_rng().gen(), 
                    green: rand::thread_rng().gen(), 
                    blue: rand::thread_rng().gen(), 
                    alpha: 1.0 
                },
                custom_size: Some(card_size),
                ..default()
            },
            ..default()
        };

        let card_text = Text2dBundle {
            text: Text::from_section("Take solace ye of little faith for today we dine in hell.", text_style.clone())
                .with_alignment(text_alignment),
            transform: Transform::from_xyz(0.0, -card_width/2.0, 100.0),
            text_2d_bounds: Text2dBounds{ size: Vec2{x:card_width, y:card_height} },
            ..default()
        };

        commands
        .spawn(sprite_bundle)
        .insert(Name::new("test_sprite"))
        .insert(Collider::cuboid(card_width/2.0, card_height/2.0))
        .insert(Sensor)
        .with_children(|x| {
            x.spawn(card_text)
            .insert(Name::new("text"));
        });
    }
}