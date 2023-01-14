use bevy::{
    ecs::system::{Command, EntityCommands},
    prelude::{
        AssetServer, BuildWorldChildren, Commands, Component, Handle, Image, ReflectComponent, Res,
        Resource, Transform, Vec2, World,
    },
    reflect::Reflect,
    sprite::{Sprite, SpriteBundle},
    text::{Text, Text2dBounds, Text2dBundle, TextAlignment, TextStyle},
};

fn main() {}

#[derive(Resource)]
pub struct CardConstructionConfig {
    size: Vec2,
    text_style: TextStyle,
    text_alignment: TextAlignment,
    card_width: f32,
    card_height: f32,
    magic_number: f32,
}

trait Constructable: 'static + Send {
    fn construct(self: Box<Self>, world: &mut World);
}

pub struct Construct {
    constructables: Vec<Box<dyn Constructable>>,
}

impl Command for Construct {
    fn write(self, world: &mut World) {
        self.constructables
            .into_iter()
            .for_each(|constructable| constructable.construct(world));
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Card {
    pub texture: Handle<Image>,
}

impl Constructable for Card {
    fn construct(self: Box<Self>, world: &mut World) {
        // assume CardConfig is in a resource
        let card_config = world.resource::<CardConstructionConfig>();
        let texture = self.texture.clone();
        world.spawn((
            *self,
            SpriteBundle {
                texture,
                sprite: Sprite {
                    custom_size: Some(card_config.size),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CardDescription {
    pub desc: String,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Name(String);

impl Constructable for CardDescription {
    fn construct(self: Box<Self>, world: &mut World) {
        let card_config = world.resource::<CardConstructionConfig>();
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
            ..Default::default()
        };
        world.spawn(*self).with_children(|ent| {
            ent.spawn((card_desc, Name("card description".to_owned())));
        });
    }
}

fn test(mut commands: Commands, asset_server: Res<AssetServer>) {
    let card: Box<dyn Constructable> = Box::new(Card {
        texture: asset_server.load("test/whatever.png"),
    });
    let card_desc: Box<dyn Constructable> = Box::new(CardDescription {
        desc: "Hi".to_owned(),
    });

    commands.add(Construct {
        constructables: vec![card, card_desc],
    });
}

