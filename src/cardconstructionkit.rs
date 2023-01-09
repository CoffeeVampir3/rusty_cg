use bevy::text::Text2dBounds;

pub use crate::*;
pub struct CardConstructionKitPlugin;

impl Plugin for CardConstructionKitPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(test_initialize)
        .add_system(run_construction)
        ;
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Card {
    pub texture: Handle<Image>
}


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CardDescription {
    pub desc: String
}

fn test_initialize(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    commands
    .spawn(Card{texture: asset_server.load("test/whatever.png"),})
    .insert(CardDescription{desc: "Hello World".to_string()})
    ;
}

fn run_construction(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut top_layer: ResMut<TopLayer>,
    unconstructed_cards: Query<(
        Entity, 
        &Card,
        Option<&CardDescription>,
    )>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let card_width = 200.0;
    let card_height = 300.0;
    let card_size = Vec2 {x:card_width, y:card_height};
    let text_alignment = TextAlignment::CENTER;
    let magic_number = 0.5/524288.0;
    let text_style = TextStyle {
        font_size: 24.0,
        font,
        color: Color::BLACK,
    };

    for (ent, card, desc) in &unconstructed_cards {
        let z_height = top_layer.top();

        let sprite_bundle = SpriteBundle {
            texture: card.texture.clone(),
            transform: Transform::from_xyz(0.0, 0.0, z_height),
            sprite: Sprite {
                custom_size: Some(card_size),
                ..default()
            },
            ..default()
        };

        commands.entity(ent).despawn_recursive();

        let mut spawned = commands.spawn(sprite_bundle);
        spawned
        .insert(Name::new("test_sprite"))
        .insert(Collider::cuboid(card_width/2.0, card_height/2.0))
        .insert(Sensor)
        ;

        if desc.is_some() {
            let card_desc = Text2dBundle {
                text: Text::from_section("Take solace ye of little faith for today we dine in hell.", text_style.clone())
                    .with_alignment(text_alignment),
                transform: Transform::from_xyz(0.0, -card_height/3.0, magic_number),
                text_2d_bounds: Text2dBounds{ size: Vec2{x:card_width, y:card_height} },
                ..default()
            };
            spawned
            .with_children(|x| {
                x.spawn(card_desc)
                .insert(Name::new("card description"));
            });
        }

        println!("Constructed a card {ent:?}");
    }
}