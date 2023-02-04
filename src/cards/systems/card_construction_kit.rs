pub use crate::*;
use bevy::utils::HashSet;

pub struct CardConstructionKitPlugin;
impl Plugin for CardConstructionKitPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<CardImage>()
            .register_type::<CardDescription>()
            .register_type::<CardCost>()
            .register_type::<CardName>()
            .register_type::<GameplayTag>()
            .register_type::<HashSet<GameplayTag>>()
            .register_type::<GameplayTagGroup>()
            .add_startup_system(initialize_construction_config)
            .add_startup_system_to_stage(StartupStage::PostStartup, test);
    }
}

#[derive(Resource)]
pub struct CardConstructionConfig {
    pub card_width: f32,
    pub card_height: f32,
    pub card_size: Vec2,
    pub text_alignment: TextAlignment,
    pub magic_number: f32,
    pub text_style: TextStyle,
}

fn initialize_construction_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraCode-Bold.ttf");
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

fn test(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    card_config: Res<CardConstructionConfig>
) {
    let fireball = FireballCard::default().make();
    let tyrant = EmpireCarnageTyrant::default().make();

    let generic = CardBase {
        name: NameConstructor { name: "Test Card".to_string() },
        desc: DescriptionConstructor { desc: "Test Card Description".to_string() },
        image: ImageConstructor { texture_path: "card_images/black_empire/orb_of_annihilation.png".to_string() },
    }.make();

    let cards = vec![fireball, tyrant, generic];

    for card in cards {
        let mut initial = commands.spawn_empty();
        for piece in card {
            piece.construct(&mut initial, &asset_server, &card_config);
        }
    }
}