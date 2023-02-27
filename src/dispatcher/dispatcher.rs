pub use crate::*;

pub struct DispatcherPlugin;
impl Plugin for DispatcherPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, on_drop);
    }
}

fn on_drop(
    world: &mut World
) {
    world.resource_scope(|world, events: Mut<Events<DropEvent>>| {
        for drop_ev in events.iter_current_update_events() {
            if let Some(q) = world.get::<Test>(drop_ev.dragged) {
                let res = (q.validate)(world, drop_ev.dropped_on);
                println!("Dropped: {:?} on {:?}. Validation was {res}", drop_ev.dragged, drop_ev.dropped_on);
            } else {
                println!("No validator supplied.");
            }
        }
    });
}