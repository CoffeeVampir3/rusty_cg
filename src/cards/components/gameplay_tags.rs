use bevy::reflect::Reflect;

#[derive(Reflect, Default, Clone, Copy, PartialEq, Hash, Eq, serde::Serialize, serde::Deserialize)]
pub enum GameplayTag {
    #[default]
    Placeholder,
    Creature,
}