pub use crate::*;

pub struct CGSys;

impl Plugin for CGSys {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(SpriteDragDrop);
    }
}