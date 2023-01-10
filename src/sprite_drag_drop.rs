pub use crate::*;
use bevy::utils::HashSet;
use bevy_rapier2d::prelude::RapierContext;
pub struct SpriteDragDrop;

impl Plugin for SpriteDragDrop {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickEvent>()
            .add_event::<DropEvent>()
            .add_event::<HoveringEvent>()
            .add_system_to_stage(CoreStage::First, handle_sprite_mouse_interactions)
            .add_system(sprite_drag)
            .add_system(debug_hover)
            .add_system_to_stage(CoreStage::Last, sprite_end_drag);
    }
}

fn debug_hover(
    mut commands: Commands,
    mut hovers: EventReader<HoveringEvent>,
    current_hovers: Query<(Entity, &HoveringOver)>,
) {
    let mut seen_hovers = HashSet::<Entity>::new();
    for ev in hovers.iter() {
        let ent = ev.ent;
        if current_hovers.get(ent).is_err() {
            commands.entity(ent).insert(HoveringOver);
            println!("Began hovering over {ent:?}");
            continue;
        } else {
            seen_hovers.insert(ent);
        }
    }

    for (ent, _) in &current_hovers {
        if seen_hovers.contains(&ent) {
            continue;
        }
        commands.entity(ent).remove::<HoveringOver>();
        println!("Ended hovering over {ent:?}");
    }
}

pub struct HoveringEvent{
    pub hover_pos: Vec2,
    pub ent: Entity,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct HoveringOver;

pub struct ClickEvent {
    pub click_pos: Vec2,
    pub ent: Entity,
}

pub struct DropEvent {
    pub drag_info: Dragging,
    pub ent: Entity,
}

#[derive(Reflect, Component, Clone, Default)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Dragging {
    pub offset: Vec2,
    pub start_pos: Vec3,
}

fn handle_sprite_mouse_interactions(
    mut commands: Commands,
    button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    sprites: Query<(Entity, &GlobalTransform), (With<Sprite>, Without<Dragging>)>,
    mut click_ev: EventWriter<ClickEvent>,
    rapier_context: Res<RapierContext>,
    mut hover_ev: EventWriter<HoveringEvent>,
) {

    let Some(window) = windows.get_primary() else {return};
    let Some(cursor_point_system) = window.cursor_position() else {return};
    let cursor_point_game = helpers::get_window_relative_cursor_pos(&window);

    let res = helpers::pointcast_2d(&rapier_context, cursor_point_game, &sprites);

    let Some((ent, xform)): Option<(Entity, &GlobalTransform)> = res else {return};

    let position = xform.translation();
    let sprite_position = position.truncate();
    let cursor_offset = sprite_position - cursor_point_system;

    if button_input.just_pressed(MouseButton::Left) {
        commands.entity(ent).insert(Dragging {
            offset: cursor_offset,
            start_pos: position,
        });
    
        click_ev.send(ClickEvent {
            click_pos: cursor_point_system,
            ent,
        });
        return;
    }

    hover_ev.send(HoveringEvent {
        ent:ent,
        hover_pos: cursor_point_system
    });
}

fn sprite_drag(
    mut mouse_moved_event: EventReader<CursorMoved>,
    mut sprites: Query<(&Dragging, &mut Transform), With<Sprite>>,
    mut top_layer: ResMut<TopLayer>,
) {
    let Some(motion) = mouse_moved_event.iter().last() else {return};

    for (dragging, mut xform) in &mut sprites {
        //TODO:: Z, There's a bug here because we keep calling top_layer.top() which is going to be a problem
        //in the future.
        xform.translation = (motion.position + dragging.offset).extend(top_layer.top());
    }
}

fn sprite_end_drag(
    mut commands: Commands,
    button_input: Res<Input<MouseButton>>,
    dragged_entities: Query<(Entity, &Dragging)>,
    mut drop_ev: EventWriter<DropEvent>,
) {
    if !button_input.just_released(MouseButton::Left) {
        return;
    }

    for (ent, drag) in &dragged_entities {
        commands.entity(ent).remove::<Dragging>();
        drop_ev.send(DropEvent {
            drag_info: drag.clone(),
            ent,
        });
    }
}
