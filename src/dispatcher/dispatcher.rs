pub use crate::*;

pub struct DispatcherPlugin;
impl Plugin for DispatcherPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PostUpdate, on_drop_trait_q);
    }
}

fn on_drop_trait_q(
    mut drop_evs: EventReader<DropEvent>,
    card_components: Query<&dyn CardComponent>
) {
    for ev in drop_evs.iter() {
        let initiator = ev.dragged;
        let target = ev.dropped_on;

        let Ok(initiator_name) = card_components.get_component::<CardName>(initiator) else {continue};
        let Ok(target_name) = card_components.get_component::<CardName>(target) else {continue};

        println!("Wew lad I: {initiator:?} {:?} T: {target:?} {:?}", initiator_name.name, target_name.name);
    }
}