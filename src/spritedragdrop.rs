use bevy_rapier2d::prelude::{RapierContext, QueryFilter};
pub use crate::*;
pub struct SpriteDragDrop;

impl Plugin for SpriteDragDrop {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ClickEvent>()
            .add_event::<DropEvent>()

            .add_system_to_stage(CoreStage::First, sprite_click)
            .add_system(sprite_drag)
            .add_system_to_stage(CoreStage::Last, sprite_end_drag)
            ;
    }
}

pub struct ClickEvent {
    pub click_pos: Vec2,
    pub ent: Entity,
}

pub struct DropEvent {
    pub drag_info: Dragging,
    pub ent: Entity
}

#[derive(Reflect, Component, Clone, Default)]
#[reflect(Component)]
#[component(storage = "SparseSet")]
pub struct Dragging {
    pub offset: Vec2,
    pub start_pos: Vec3,
}

fn sprite_click(
    mut commands: Commands,
    button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    sprites: Query<(
        Entity,
        &GlobalTransform,
    ), With<Sprite>>,
    mut click_ev: EventWriter<ClickEvent>,
    rapier_context: Res<RapierContext>,
) {
    if !button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(window) = windows.get_primary() else {return};
    let cursor_point_system = window.cursor_position().unwrap();
    let cursor_point_game = helpers::get_window_relative_cursor_pos(&window);

    let mut max = f32::NEG_INFINITY;
    let mut res: Option<(Entity, &GlobalTransform)> = None;
    rapier_context.intersections_with_point(cursor_point_game, QueryFilter::default(), 
    |x| {
        let Ok((ent, xform)) = sprites.get(x) else {return true};

        let ord = xform.translation().z;

        if ord >= max {
            res = Some((ent, xform));
            max = ord;
        }
        true
    });
    
    let Some((ent, xform)): Option<(Entity, &GlobalTransform)> = res else {return};

    let position = xform.translation();
    let sprite_position = position.truncate();
    let cursor_offset = sprite_position - cursor_point_system;

    commands.entity(ent)
    .insert(Dragging {
        offset: cursor_offset,
        start_pos: position,
    });

    click_ev.send(ClickEvent { click_pos: cursor_point_system, ent })
}

fn sprite_drag(
    mut mouse_moved_event: EventReader<CursorMoved>,
    mut sprites: Query<(&Dragging, &mut Transform), With<Sprite>>,
    mut top_layer: ResMut<TopLayer>
) {
    let Some(motion) = mouse_moved_event.iter().last() else {return};

    for (dragging, mut xform) in &mut sprites {
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
        drop_ev.send(DropEvent { drag_info: drag.clone(), ent});
    }
}