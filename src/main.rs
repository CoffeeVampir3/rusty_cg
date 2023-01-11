mod card_components;
mod card_construction_kit;
mod cg_sys;
mod helpers;
mod sprite_interactions;
mod sprite_layers;
pub use card_construction_kit::*;
pub use cg_sys::*;
pub use sprite_interactions::*;
pub use sprite_layers::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use bevy_egui::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_inspector_egui_rapier::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugin(InspectableRapierPlugin)
        //.add_plugin(WorldInspectorPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(SpriteLayerSystem)
        .add_plugin(CGCorePlugin)
        .add_plugin(CGSys)
        .add_plugin(CardConstructionKitPlugin)
        .add_system(ui_example)
        .run();
}

pub struct CGCorePlugin;

impl Plugin for CGCorePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello")
        .title_bar(false)
        .resizable(false)
        .fixed_pos(egui::Pos2::new(640.0, 360.0))
        .show(egui_context.ctx_mut(), |ui| {
            ui.label("This is a card description peepo poggers");
        });
}

fn setup(mut commands: Commands) {
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

    // commands
    //     .spawn(test_drop_zone)
    //     .insert(Collider::cuboid(400.0, 100.0))
    //     .insert(Sensor);
}
