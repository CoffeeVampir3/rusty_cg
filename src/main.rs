mod spritedragdrop;
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
    .add_plugin(SpriteDragDrop)
    .run();
}

pub struct CGCorePlugin;

impl Plugin for CGCorePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    for x in (-2..=2).map(|x| x as f32 * 125.0) {
        commands
        .spawn(SpriteBundle {
            texture: asset_server.load("test/whatever.png"),
            transform: Transform::from_xyz(x, -300.0, (x / 125.0) + 8.0),
            sprite: Sprite {
                color: Color::Rgba { 
                    red: rand::thread_rng().gen() , 
                    green: rand::thread_rng().gen() , 
                    blue: rand::thread_rng().gen() , 
                    alpha: 1.0 
                },
                custom_size: Some(Vec2 { x: 150.0, y: 275.0 }),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("test_sprite"))
        .insert(Collider::cuboid(75.0, 137.5))
        .insert(Sensor)
        ;
    }
}