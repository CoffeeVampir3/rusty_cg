use bevy::{prelude::*};
use bevy_rapier2d::{prelude::{RapierContext, QueryFilter}};

pub struct SpriteDragDrop;

impl Plugin for SpriteDragDrop {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ClickEvent>()
            .add_event::<DropEvent>()

            .add_startup_system(debug)
            .add_system_to_stage(CoreStage::First, sprite_click)
            .add_system(sprite_drag)
            .add_system(debug_sprite_clicks)
            .add_system(debug_sprite_drop)
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
    pub start_pos: Vec2,
    pub orig_z: f32
}

//TODO::Z, DEBUG!
//DEBUG!
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
struct DebugMouseposDebugger;

fn debug(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
    .spawn(SpriteBundle {
        texture: asset_server.load("test/whatever.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2 { x: 30.0, y: 30.0 }),
            ..default()
        },
        ..default()
    })
    .insert(Name::new("debug"))
    .insert(DebugMouseposDebugger)
    ;
}

fn sprite_click(
    mut commands: Commands,
    button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    sprites: Query<(
        Entity,
        &GlobalTransform,
    )>,
    mut click_ev: EventWriter<ClickEvent>,
    rapier_context: Res<RapierContext>,
    mut debug: Query<(&DebugMouseposDebugger, &mut Transform)>
) {
    if !button_input.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(location) = windows.get_primary() else {return};
    let Some(cursor_pos) = location.cursor_position() else {return};

    let window_size = Vec2 {
        x: location.width(),
        y: location.height(),
    };

    let cursor_point = cursor_pos - window_size / 2.0;

    //TODO:: @Z Debug!
    //DEBUG!
    let (_, mut dbgxform) = debug.single_mut();
    dbgxform.translation = cursor_point.extend(20.0);

    let mut max = f32::NEG_INFINITY;
    let mut res: Option<(Entity, &GlobalTransform)> = None;
    rapier_context.intersections_with_point(cursor_point, QueryFilter::default(), 
    |x| {
        let Ok((ent, xform)) = sprites.get(x) else {return true};

        let ord = xform.translation().z;

        if ord >= max {
            res = Some((ent, xform));
            max = ord;
        }
        true
    });

    let Some((ent, xform)) = res else {return};

    let position = xform.translation();
    let sprite_position = position.truncate();
    let cursor_offset = sprite_position - cursor_pos;

    commands.entity(ent)
    .insert(Dragging {
        offset: cursor_offset,
        start_pos: sprite_position,
        orig_z: xform.translation().z,
    });

    click_ev.send(ClickEvent { click_pos: cursor_pos, ent })
}

fn sprite_drag(
    mut mouse_moved_event: EventReader<CursorMoved>,
    mut sprites: Query<(&Dragging, &mut Transform), With<Sprite>>,
) {
    let Some(motion) = mouse_moved_event.iter().last() else {return};

    for (dragging, mut xform) in &mut sprites {
        xform.translation = (motion.position + dragging.offset).extend(100.0);
    }
}

fn sprite_end_drag(
    mut commands: Commands,
    button_input: Res<Input<MouseButton>>,
    mut dragged_entities: Query<(Entity, &Dragging, &mut Transform)>,
    mut drop_ev: EventWriter<DropEvent>,
) {
    if !button_input.just_released(MouseButton::Left) {
        return;
    }

    for (ent, drag, mut xform) in &mut dragged_entities {
        commands.entity(ent).remove::<Dragging>();
        drop_ev.send(DropEvent { drag_info: drag.clone(), ent});
        xform.translation.z = drag.orig_z;
        println!("Ended drag: {:?}", ent);
    }
}

fn debug_sprite_clicks(
    clickable_entities: Query<(Entity, &Sprite)>,
    mut click_ev: EventReader<ClickEvent>,
) {
    for ev in click_ev.iter() {
        let (ent, _) = clickable_entities.get(ev.ent).unwrap();
        println!("Clicked: {:?}", ent);
    }
}

fn debug_sprite_drop(
    mut dropped_entities: Query<(Entity, &mut Transform)>,
    mut drop_ev: EventReader<DropEvent>,
) {
    for ev in drop_ev.iter() {
        let (ent, mut xform) = dropped_entities.get_mut(ev.ent).unwrap();
        xform.translation = ev.drag_info.start_pos.extend(xform.translation.z);
        println!("Dropped: {:?}", ent);
    }
}