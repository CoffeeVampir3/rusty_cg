use bevy::prelude::*;
use bevy_rapier2d::prelude::{RapierContext, QueryFilter};

pub struct SpriteDragDrop;

impl Plugin for SpriteDragDrop {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ClickEvent>()
            .add_event::<DropEvent>()
            .insert_resource(TopLayer { current:1.0 })

            .add_startup_system(debug)
            .add_system_to_stage(CoreStage::First, sprite_click)
            .add_system(sprite_drag)
            .add_system_to_stage(CoreStage::Last, sprite_end_drag)
            ;
    }
}

#[derive(Resource)]
pub struct TopLayer {
    current: f32
}

impl TopLayer {
    fn top(&mut self) -> f32 {
        let cur = self.current;
        //2^19, abitarily small power of 2. This acts as an "epsilon" expression to move the float up a small increment.
        self.current += self.current / 524288.0;
        cur
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
    });

    click_ev.send(ClickEvent { click_pos: cursor_pos, ent })
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