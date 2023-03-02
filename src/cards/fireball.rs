pub use crate::*;

#[derive(Clone)]
pub struct FireballCard {
    pub base: CardBase,
}

impl Default for FireballCard {
    fn default() -> Self {
        FireballCard {
            base: CardBase {
                image: ImageConstructor {texture_path:"test/focalize.png".to_string()},
                name: NameConstructor{name:"Fireball".to_string()},
                desc: DescriptionConstructor{desc:"Fireball Description".to_string()},
            },
        }
    }
}

impl Descriptor for FireballCard {
    fn make(&self) -> Vec::<Box<dyn Constructable>> {
        let mut all_parts = Vec::<Box<dyn Constructable>>::new();

        all_parts.append(&mut self.base.make());

        all_parts
    }
}