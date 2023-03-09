pub use crate::*;

pub struct CardConstructionKitPlugin;
impl Plugin for CardConstructionKitPlugin {
    fn build(&self, app: &mut App) {
        use bevy_trait_query::RegisterExt;
        register_types_and_components!(app, 
            CardCost,
            CardDescription,  
            GameplayTagGroup, 
            CardImage, 
            CardName);
        app.add_startup_system(initialize_construction_config);
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
        text_alignment: TextAlignment::Center,
        magic_number: 0.5 / 524288.0,
        text_style: TextStyle {
            font_size: 24.0,
            font: font,
            color: Color::BLACK,
        },
    });
}