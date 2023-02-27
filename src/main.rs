mod cards;
mod interactions;
mod dispatcher;
pub use cards::*;
pub use interactions::*;
use bevy::{prelude::*};
use bevy_egui::*;
use interactions::cards::dispatcher::DispatcherPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(SpriteLayerSystem)
        .add_plugin(CGCorePlugin)
        .add_plugin(CardConstructionKitPlugin)
        .add_plugin(SpriteInteractionPlugin)
        .add_plugin(DispatcherPlugin)
        .run();
}

pub struct CGCorePlugin;

impl Plugin for CGCorePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, setup);
        //.add_system(test_trait_query);
    }
}

fn test_trait_query(
    components: Query<&dyn CardComponent>
) {
    for comps in &components {
        for comp in comps {
            println!("{:?}", comp.get_name());
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    card_config: Res<CardConstructionConfig>,
    windows: Res<Windows>,
) {
    let Some(window) = windows.get_primary() else {return;};
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
        .insert(GameplayTagGroup::default());

    make_test_hand(commands, &asset_server, &card_config, window);

}

#[derive(Component)]
pub struct Test {
    pub validate: fn(&mut World, target:Entity)->bool
}

fn validate(world: &mut World, target: Entity) -> bool {
    let Some(tags) = world.get::<GameplayTagGroup>(target) else {return false};
    tags.all(&[GameplayTag::Creature])
}

fn make_test_hand(mut commands: Commands, asset_server: &AssetServer, card_config: &CardConstructionConfig, window: &Window) {
    let fireball = FireballCard::default().make();
    let tyrant = EmpireCarnageTyrant::default().make();

    let mut generic = CardBase {
        name: NameConstructor { name: "Test Card".to_string() },
        desc: DescriptionConstructor { desc: "Test Card Description".to_string() },
        image: ImageConstructor { texture_path: "test/young-magi.png".to_string() },
    }.make();

    let tg = Box::new(GameplayTagGroupConstructor::new(&[GameplayTag::Creature]));
    generic.push(tg);

    let cards = vec![fireball, tyrant, generic];

    let mut i = -1;
    for card in cards {
        let mut initial = commands.spawn_empty();
        for piece in card {
            piece.construct(&mut initial, &asset_server, &card_config);
        }
        initial.insert(TransformBundle {
            local: Transform {
                translation: Vec3::new(i as f32 * card_config.card_width, (-window.height() + card_config.card_height)/2.0, 0.0),
                ..default()
            },
            ..default()
        });
        i += 1;
    }
}