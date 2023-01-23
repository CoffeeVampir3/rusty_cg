pub use crate::*;
use bevy::utils::HashSet;
pub use card_components::*;

pub struct CardConstructionKitPlugin;
impl Plugin for CardConstructionKitPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Card>()
            .register_type::<CardDescription>()
            .register_type::<CardCost>()
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
    let asuka_tex = asset_server.load("card_images/black_empire/blood_emperor.png");
    let kekw_tex = asset_server.load("card_images/black_empire/disembody.png");

    let asuka_card = CardConstructor {texture: asuka_tex};
    let kekw_card = CardConstructor {texture: kekw_tex};
    let desc = CardDescriptionConstructor {description: CardDescription{desc: "Hello World".to_string()}};
    let cost = CardCostConstructor {cost: CardCost{cost:5}};

    use GameplayTag::*;
    let test_tags = GameplayTagGroup::new(&[Creature, Placeholder]);

    let mut initial = commands.spawn_empty();

    asuka_card.construct(&mut initial, &card_config);
    desc.construct(&mut initial, &card_config);
    cost.construct(&mut initial, &card_config);
    initial.insert(test_tags);

    let mut initial = commands.spawn_empty();

    kekw_card.construct(&mut initial, &card_config);
    desc.construct(&mut initial, &card_config);

    initial.insert(Name::new("This Is a Different Name"));
}