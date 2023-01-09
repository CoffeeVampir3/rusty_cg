mod cardconstructionkit;
mod cgsys;
mod helpers;
mod spritedragdrop;
mod spritelayers;
pub use cardconstructionkit::*;
pub use cgsys::*;
pub use spritedragdrop::*;
pub use spritelayers::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui_rapier::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(InspectableRapierPlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(SpriteLayerSystem)
        .add_plugin(CGCorePlugin)
        .add_plugin(CGSys)
        .add_plugin(CardConstructionKitPlugin)
        .run();
}

pub struct CGCorePlugin;

impl Plugin for CGCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut top_layer: ResMut<TopLayer>) {
    commands.spawn(Camera2dBundle::default());

    let test_drop_zone = SpriteBundle {
        //texture: asset_server.load("test/whatever.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite {
            color: Color::Rgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.2,
            },
            custom_size: Some(Vec2 { x: 800.0, y: 200.0 }),
            ..default()
        },
        ..default()
    };

    commands
        .spawn(test_drop_zone)
        .insert(Collider::cuboid(400.0, 100.0))
        .insert(Sensor);
}
