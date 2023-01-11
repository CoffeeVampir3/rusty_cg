pub use crate::*;
pub use card_components::*;
pub struct CardConstructionKitPlugin;

impl Plugin for CardConstructionKitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_construction_config)
            .add_startup_system(test_initialize)
            .add_system(run_construction);
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

fn test_initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Card {
            texture: asset_server.load("test/whatever.png"),
        })
        .insert(CardDescription {
            desc: "Hello World".to_string(),
        });
}

fn run_construction(
    mut commands: Commands,
    unconstructed_cards: Query<(Entity, &Card, Option<&CardDescription>)>,
    card_config: Res<CardConstructionConfig>,
) {
    for (ent, card, desc) in &unconstructed_cards {
        commands.entity(ent).despawn_recursive();

        let mut spawned = commands.spawn(card.construct(card_config.as_ref()));
        spawned
            .insert(Name::new("test_sprite"))
            .insert(Collider::cuboid(
                card_config.card_width / 2.0,
                card_config.card_height / 2.0,
            ))
            .insert(Sensor)
            .insert(Interactable::default());

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
