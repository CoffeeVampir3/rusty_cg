use bevy::{prelude::*, window::Window};

pub fn get_window_relative_cursor_pos(wnd: &Window) -> Option<Vec2> {
    let Some(cursor_pos) = wnd.cursor_position() else {return None};

    let window_size = Vec2 {
        x: wnd.width(),
        y: wnd.height(),
    };

    Some(cursor_pos - window_size / 2.0)
}

pub fn pointcast_2d<'a>(
    cursor_point_game: Vec2,
    sprites: &'a Query<(Entity, &Sprite, &GlobalTransform)>,
    excluded_entity: Option<Entity>,
) -> Option<(Entity, &'a Sprite, &'a GlobalTransform)> {
    let mut highest_z = f32::NEG_INFINITY;
    let mut ret: Option<(Entity, &'a Sprite, &'a GlobalTransform)> = None;

    for (ent, sprite, xform) in sprites {
        if let Some(excluded) = excluded_entity {
            if ent == excluded {
                continue;
            }
        }

        let Some(size) = sprite.custom_size else {continue};

        let xform_z = xform.translation().z;
        if xform_z < highest_z {
            continue;
        }

        let initial_x = xform.translation().x - (0.5 * size.x);
        let initial_y = xform.translation().y - (0.5 * size.y);

        let terminal_x = initial_x + size.x;
        let terminal_y = initial_y + size.y;
        
        if (initial_x..=terminal_x).contains(&cursor_point_game.x) && (initial_y..=terminal_y).contains(&cursor_point_game.y) {
            highest_z = xform_z;
            ret = Some((ent, sprite, xform));
        }
    }

    ret
}