pub use crate::*;

#[derive(Clone)]
pub struct EmpireCarnageTyrant {
    creature: CreatureBase,
    base: CardBase,
}

impl Default for EmpireCarnageTyrant {
    fn default() -> Self {
        EmpireCarnageTyrant {
            creature: CreatureBase {
                tags: GameplayTagGroupConstructor::new(&[GameplayTag::Creature])
            },
            base: CardBase {
                image: ImageConstructor {texture_path:"test/disintegrate.png".to_string()},
                name: NameConstructor{name:"Carnage Tyrant".to_string()},
                desc: DescriptionConstructor{desc:"Carnage Tyrant Description".to_string()},
            },
        }
    }
}

impl Descriptor for EmpireCarnageTyrant {
    fn make(&self) -> Vec::<Box<dyn Constructable>> {
        let clone = self.clone();

        let mut all_parts = Vec::<Box<dyn Constructable>>::new();

        all_parts.append(&mut clone.creature.make());
        all_parts.append(&mut clone.base.make());

        all_parts
    }
}