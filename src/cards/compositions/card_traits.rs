pub use crate::*;

pub trait Descriptor {
    fn make(&self) -> Vec::<Box<dyn Constructable>>;
}