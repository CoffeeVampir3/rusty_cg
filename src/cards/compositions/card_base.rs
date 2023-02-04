use std::vec;
pub use crate::*;

#[derive(Clone)]
pub struct CardBase {
    pub name: NameConstructor,
    pub desc: DescriptionConstructor,
    pub image: ImageConstructor,
}

impl Descriptor for CardBase {
    fn make(&self) -> Vec::<Box<dyn Constructable>> {
        let clone = self.clone();
        let constructables: Vec::<Box<dyn Constructable>> = vec!{Box::new(clone.image), Box::new(clone.name), Box::new(clone.desc)};
        constructables
    }
}