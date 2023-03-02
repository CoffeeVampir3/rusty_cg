pub use crate::*;

use self::structures::NetSet;

#[derive(Default, Clone)]
pub struct GameplayTagGroupConstructor {
    tags: NetSet::<GameplayTag>
}

impl GameplayTagGroupConstructor {
    pub fn new(new_tags: &[GameplayTag]) -> Self {
        Self { tags: NetSet::<GameplayTag>::new(new_tags) } 
    }
}

impl Constructable for GameplayTagGroupConstructor {
    fn construct(&self, cmds: &mut bevy::ecs::system::EntityCommands, _: &AssetServer, _: &CardConstructionConfig) {
        cmds.insert(GameplayTagGroup::new(self.tags.clone()));
    }
}

#[derive(Reflect, Component, Default, Clone, serde::Serialize, serde::Deserialize)]
#[reflect(Component)]
pub struct GameplayTagGroup {
    tags: NetSet::<GameplayTag>
}

impl GameplayTagGroup {
    pub fn new(tags: NetSet::<GameplayTag>) -> Self { Self { tags } }
}

impl CardComponent for GameplayTagGroup{
    fn get_name(&self) -> String {
        "Tags".to_string()
    }
}