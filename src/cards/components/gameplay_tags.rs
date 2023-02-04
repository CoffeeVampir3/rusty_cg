use bevy::reflect::Reflect;

#[derive(Default, Clone, Copy, PartialEq, Hash, Eq, Reflect)]
pub enum GameplayTag {
    #[default]
    Placeholder,
    Creature,
    TestThird
}