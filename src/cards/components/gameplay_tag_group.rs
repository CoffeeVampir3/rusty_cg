use bevy::utils::HashSet;
pub use crate::*;

#[derive(Default, Clone)]
pub struct GameplayTagGroupConstructor {
    tags: HashSet::<GameplayTag>
}

impl GameplayTagGroupConstructor {
    pub fn new(new_tags: &[GameplayTag]) -> Self {
        let mut tags = HashSet::<GameplayTag>::default();
        for tag in new_tags {
            tags.insert(*tag);
        }
        Self { tags } 
    }
}

impl Constructable for GameplayTagGroupConstructor {
    fn construct(&self, cmds: &mut bevy::ecs::system::EntityCommands, _: &AssetServer, _: &CardConstructionConfig) {
        cmds.insert(GameplayTagGroup::new(&self));
    }
}

#[derive(Reflect, Component, Default, Clone, serde::Serialize, serde::Deserialize)]
#[reflect(Component)]
pub struct GameplayTagGroup {
    tags: HashSet::<GameplayTag>
}

impl CardComponent for GameplayTagGroup{
    fn get_name(&self) -> String {
        "Tags".to_string()
    }
}

impl GameplayTagGroup {
    pub fn new(new_tags: &GameplayTagGroupConstructor) -> Self {
        Self { tags:  new_tags.tags.clone()} 
    }

    #[cfg(test)]
    fn make_test_set(test_tags: &[GameplayTag]) -> Self {
        let mut tags = HashSet::<GameplayTag>::default();
        for tag in test_tags {
            tags.insert(*tag);
        }
        Self { tags } 
    }

    pub fn add_tag(&mut self, tag: GameplayTag) {
        self.tags.insert(tag);
    }

    pub fn add_tags(&mut self, tags: &[GameplayTag]) {
        for tag in tags {
            self.tags.insert(*tag);
        }
    }

    pub fn remove_tag(&mut self, tag: &GameplayTag) {
        self.tags.remove(tag);
    }

    pub fn remove_tags(&mut self, tags: &[GameplayTag]) {
        for tag in tags {
            self.tags.remove(tag);
        }
    }

    pub fn contains(&self, tag: &GameplayTag) -> bool {
        self.tags.contains(tag)
    }

    pub fn all(&self, tags: &[GameplayTag]) -> bool {
        for tag in tags {
            if !self.contains(tag) {
                return false
            }
        }
        true
    }

    pub fn any(&self, tags: &[GameplayTag]) -> bool {
        for tag in tags {
            if self.contains(tag) {
                return true
            }
        }
        false
    }

    pub fn none(&self, tags: &[GameplayTag]) -> bool {
        let mut res = false;
        for tag in tags {
            res |= self.contains(tag);
        }
        !res
    }

    pub fn all_g(&self, tags: &GameplayTagGroup) -> bool {
        for tag in &tags.tags {
            if !self.tags.contains(&tag) {
                return false
            }
        }
        true
    }

    pub fn any_g(&self, tags: &GameplayTagGroup) -> bool {
        for tag in &tags.tags {
            if self.tags.contains(&tag) {
                return true
            }
        }
        false
    }

    pub fn none_g(&self, tags: &GameplayTagGroup) -> bool {
        let mut res = false;
        for tag in &tags.tags {
            res |= self.tags.contains(tag);
        }
        !res
    }
}

#[cfg(test)]
mod tag_tests {
    use crate::*;

    #[test]
    fn gameplay_tag_group_test_all() {
        use GameplayTag::*;
        let test_tags = GameplayTagGroup::make_test_set(&[TestTwo, TestOne]);
        let test_tags2 = GameplayTagGroup::make_test_set(&[TestOne]);
        let test_tags3 = GameplayTagGroup::make_test_set(&[TestThree]);
    
        assert_eq!(test_tags.all_g(&test_tags), true, "Failed all self test.");
        assert_eq!(test_tags.all(&[TestTwo, TestOne]), true, "Failed all TestTwo TestOne");
        assert_eq!(test_tags.all(&[TestTwo]), true, "Failed all TestTwo");
    
        assert_eq!(test_tags.all_g(&test_tags2), true, "Failed all test tags 1 V test tags 2");
        assert_eq!(test_tags2.all_g(&test_tags), false, "Failed all test tags 2 V test tags 1");
        assert_eq!(test_tags2.all(&[TestTwo, TestOne]), false, "Failed all TestTwo TestOne.");
    
        assert_eq!(test_tags2.all(&[TestThree]), false, "Failed all third.");
        assert_eq!(test_tags2.all_g(&test_tags3), false, "Failed all third g.");
    }
    
    #[test]
    fn gameplay_tag_group_test_any() {
        use GameplayTag::*;
        let test_tags = GameplayTagGroup::make_test_set(&[TestTwo, TestOne]);
        let test_tags2 = GameplayTagGroup::make_test_set(&[TestOne]);
        let test_tags3 = GameplayTagGroup::make_test_set(&[TestThree]);
    
        assert_eq!(test_tags.any_g(&test_tags), true, "Failed any self test.");
        assert_eq!(test_tags.any(&[TestTwo, TestOne]), true, "Failed any TestTwo TestOne");
        assert_eq!(test_tags.any(&[TestTwo]), true, "Failed any TestTwo");
    
        assert_eq!(test_tags.any_g(&test_tags2), true, "Failed any test tags 1 V test tags 2");
        assert_eq!(test_tags2.any_g(&test_tags), true, "Failed any test tags 2 V test tags 1");
        assert_eq!(test_tags2.any(&[TestTwo, TestOne]), true, "Failed any TestTwo TestOne.");
    
        assert_eq!(test_tags2.any(&[TestThree]), false, "Failed any third.");
        assert_eq!(test_tags2.any_g(&test_tags3), false, "Failed any third g.");
    }
    
    #[test]
    fn gameplay_tag_group_test_none() {
        use GameplayTag::*;
        let test_tags = GameplayTagGroup::make_test_set(&[TestTwo, TestOne]);
        let test_tags2 = GameplayTagGroup::make_test_set(&[TestOne]);
        let test_tags3 = GameplayTagGroup::make_test_set(&[TestThree]);
    
        assert_eq!(test_tags.none_g(&test_tags), false, "Failed none self test.");
        assert_eq!(test_tags.none(&[TestTwo, TestOne]), false, "Failed none TestTwo TestOne");
        assert_eq!(test_tags.none(&[TestTwo]), false, "Failed none TestTwo");
    
        assert_eq!(test_tags.none_g(&test_tags2), false, "Failed none test tags 1 V test tags 2");
        assert_eq!(test_tags2.none_g(&test_tags), false, "Failed none test tags 2 V test tags 1");
        assert_eq!(test_tags2.none(&[TestTwo, TestOne]), false, "Failed none TestTwo TestOne.");
    
        assert_eq!(test_tags2.none(&[TestThree]), true, "Failed none third.");
        assert_eq!(test_tags2.none_g(&test_tags3), true, "Failed none third g.");
    }
}
