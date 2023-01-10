pub use crate::*;
use bevy::utils::HashSet;
use bevy_rapier2d::prelude::RapierContext;
pub struct SpriteInteractionPlugin;

impl Plugin for SpriteInteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ClickEvent>()
            .add_event::<DropEvent>()
            .add_event::<HoveringEvent>()
            .add_event::<HoverBeginEvent>()
            .add_event::<HoverEndEvent>()

            .add_system_to_stage(CoreStage::First, handle_sprite_mouse_interactions)
            .add_system(handle_sprite_begin_drag)
            .add_system(handle_sprite_drag)
            .add_system(handle_sprite_hover_events)
            .add_system_to_stage(CoreStage::Last, handle_sprite_end_drag)

            .add_system(debug_hover_begin)
            .add_system(debug_hover_end)

            ;
    }
}

fn debug_hover_begin(
    mut hover_begin_ev: EventReader<HoverBeginEvent>,
) {
    for ev in hover_begin_ev.iter() {
        let ent = ev.ent;
        println!("Began hovering over {ent:?}");
    }
}

fn debug_hover_end(
    mut hover_end_ev: EventReader<HoverEndEvent>,
) {
    for ev in hover_end_ev.iter() {
        let ent = ev.ent;
        println!("Ended hovering over {ent:?}");
    }
}

pub struct HoveringEvent{
    pub hover_pos: Vec2,
    pub ent: Entity,
}

pub struct HoverBeginEvent {
    pub hover_pos: Vec2,
    pub ent: Entity,
}

pub struct HoverEndEvent {
    pub ent: Entity,
}

pub struct ClickEvent {
    pub click_pos: Vec2,
    pub ent: Entity,
}

pub struct DropEvent {
    pub drag_info: Dragging,
    pub ent: Entity,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct HoveringOver;

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
    dragging: Query<&Dragging>,
    mut click_ev: EventWriter<ClickEvent>,
    rapier_context: Res<RapierContext>,
    mut hover_ev: EventWriter<HoveringEvent>,
) {
    if dragging.iter().len() > 0 {return};
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

fn handle_sprite_hover_events(
    mut commands: Commands,
    mut hover_persist_ev: EventReader<HoveringEvent>,
    mut hover_begin_ev: EventWriter<HoverBeginEvent>,
    mut hover_end_ev: EventWriter<HoverEndEvent>,
    current_hovers: Query<(Entity, &HoveringOver)>,
) {
    let mut seen_hovers = HashSet::<Entity>::new();
    for ev in hover_persist_ev.iter() {
        let ent = ev.ent;
        if current_hovers.get(ent).is_err() {
            commands.entity(ent).insert(HoveringOver);
            hover_begin_ev.send(HoverBeginEvent { hover_pos: (ev.hover_pos), ent: (ent) });
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
        hover_end_ev.send(HoverEndEvent { ent });
    }
}

fn handle_sprite_begin_drag(
    mut sprites: Query<(&Dragging, &mut Transform), (With<Sprite>, Added<Dragging>)>,
    mut top_layer: ResMut<TopLayer>,
) {
    for (_, mut xform) in &mut sprites {
        //TODO:: Z, There's a bug here because we keep calling top_layer.top() which is going to be a problem
        //in the future.
        xform.translation = xform.translation.truncate().extend(top_layer.top());
    }
}

fn handle_sprite_drag(
    mut mouse_moved_event: EventReader<CursorMoved>,
    mut sprites: Query<(&Dragging, &mut Transform), With<Sprite>>,
) {
    let Some(motion) = mouse_moved_event.iter().last() else {return};

    for (dragging, mut xform) in &mut sprites {
        xform.translation = (motion.position + dragging.offset).extend(xform.translation.z);
    }
}

fn handle_sprite_end_drag(
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
