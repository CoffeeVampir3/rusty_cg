use std::vec;
pub use crate::*;

#[derive(Clone)]
pub struct CreatureBase {
    pub tags: GameplayTagGroupConstructor,
}

impl Descriptor for CreatureBase {
    fn make(&self) -> Vec::<Box<dyn Constructable>> {
        let clone = self.clone();
        let constructables: Vec::<Box<dyn Constructable>> = vec!{Box::new(clone.tags)};
        constructables
    }
}