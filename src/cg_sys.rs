use bevy::utils::HashSet;
pub use crate::*;

pub struct CGSys;

impl Plugin for CGSys {
    fn build(&self, app: &mut App) {
        app.add_plugin(SpriteInteractionPlugin);
    }
}

#[derive(Default, Clone, Copy, PartialEq, Hash, Eq, Reflect)]

pub enum GameplayTag {
    #[default]
    Placeholder,
    Creature,
    TestThird
}

#[derive(Component, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct GameplayTagGroup {
    tags: HashSet::<GameplayTag>
}

impl GameplayTagGroup {
    pub fn new(new_tags: &[GameplayTag]) -> Self {
        let mut tags = HashSet::<GameplayTag>::default();
        for tag in new_tags {
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

#[test]
fn gameplay_tag_group_test_all() {
    use GameplayTag::*;
    let test_tags = GameplayTagGroup::new(&[Creature, Placeholder]);
    let test_tags2 = GameplayTagGroup::new(&[Placeholder]);
    let test_tags3 = GameplayTagGroup::new(&[TestThird]);

    assert_eq!(test_tags.all_g(&test_tags), true, "Failed all self test.");
    assert_eq!(test_tags.all(&[Creature, Placeholder]), true, "Failed all Creature Placeholder");
    assert_eq!(test_tags.all(&[Creature]), true, "Failed all Creature");

    assert_eq!(test_tags.all_g(&test_tags2), true, "Failed all test tags 1 V test tags 2");
    assert_eq!(test_tags2.all_g(&test_tags), false, "Failed all test tags 2 V test tags 1");
    assert_eq!(test_tags2.all(&[Creature, Placeholder]), false, "Failed all creature placeholder.");

    assert_eq!(test_tags2.all(&[TestThird]), false, "Failed all third.");
    assert_eq!(test_tags2.all_g(&test_tags3), false, "Failed all third g.");
}

#[test]
fn gameplay_tag_group_test_any() {
    use GameplayTag::*;
    let test_tags = GameplayTagGroup::new(&[Creature, Placeholder]);
    let test_tags2 = GameplayTagGroup::new(&[Placeholder]);
    let test_tags3 = GameplayTagGroup::new(&[TestThird]);

    assert_eq!(test_tags.any_g(&test_tags), true, "Failed any self test.");
    assert_eq!(test_tags.any(&[Creature, Placeholder]), true, "Failed any Creature Placeholder");
    assert_eq!(test_tags.any(&[Creature]), true, "Failed any Creature");

    assert_eq!(test_tags.any_g(&test_tags2), true, "Failed any test tags 1 V test tags 2");
    assert_eq!(test_tags2.any_g(&test_tags), true, "Failed any test tags 2 V test tags 1");
    assert_eq!(test_tags2.any(&[Creature, Placeholder]), true, "Failed any creature placeholder.");

    assert_eq!(test_tags2.any(&[TestThird]), false, "Failed any third.");
    assert_eq!(test_tags2.any_g(&test_tags3), false, "Failed any third g.");
}

#[test]
fn gameplay_tag_group_test_none() {
    use GameplayTag::*;
    let test_tags = GameplayTagGroup::new(&[Creature, Placeholder]);
    let test_tags2 = GameplayTagGroup::new(&[Placeholder]);
    let test_tags3 = GameplayTagGroup::new(&[TestThird]);

    assert_eq!(test_tags.none_g(&test_tags), false, "Failed none self test.");
    assert_eq!(test_tags.none(&[Creature, Placeholder]), false, "Failed none Creature Placeholder");
    assert_eq!(test_tags.none(&[Creature]), false, "Failed none Creature");

    assert_eq!(test_tags.none_g(&test_tags2), false, "Failed none test tags 1 V test tags 2");
    assert_eq!(test_tags2.none_g(&test_tags), false, "Failed none test tags 2 V test tags 1");
    assert_eq!(test_tags2.none(&[Creature, Placeholder]), false, "Failed none creature placeholder.");

    assert_eq!(test_tags2.none(&[TestThird]), true, "Failed none third.");
    assert_eq!(test_tags2.none_g(&test_tags3), true, "Failed none third g.");
}