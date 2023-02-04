pub use crate::*;

pub struct CGSys;

impl Plugin for CGSys {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(SpriteInteractionPlugin)
        .add_system(on_card_dropped);
    }
}

fn on_card_dropped(
    mut reader: EventReader<DropEvent>,
) {
    for ev in reader.iter() {
        println!("Card: {:?} dropped on {:?}", ev.held_ent, ev.dropped_ent);
    }
}