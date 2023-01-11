use bevy::{prelude::*, window::Window};
use bevy_rapier2d::prelude::{QueryFilter, RapierContext};

pub fn get_window_relative_cursor_pos(wnd: &Window) -> Option<Vec2> {
    let Some(cursor_pos) = wnd.cursor_position() else {return None};

    let window_size = Vec2 {
        x: wnd.width(),
        y: wnd.height(),
    };

    Some(cursor_pos - window_size / 2.0)
}

pub fn pointcast_2d<'a>(
    rapier_context: &RapierContext,
    cursor_point_game: Vec2,
    sprites: &'a Query<(Entity, &GlobalTransform), With<Sprite>>,
    filter: QueryFilter,
) -> Option<(Entity, &'a GlobalTransform)> {
    let mut max = f32::NEG_INFINITY;
    let mut res: Option<(Entity, &GlobalTransform)> = None;
    rapier_context.intersections_with_point(cursor_point_game, filter, |x| {
        let Ok((ent, xform)) = sprites.get(x) else {return true};

        let ord = xform.translation().z;

        if ord >= max {
            res = Some((ent, xform));
            max = ord;
        }
        true
    });
    res
}
