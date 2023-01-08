use bevy::{window::Window, prelude::*};
use bevy_rapier2d::prelude::{RapierContext, QueryFilter};

pub fn get_window_relative_cursor_pos(wnd: &Window) -> Vec2 {
    let cursor_pos = wnd.cursor_position().unwrap();

    let window_size = Vec2 {
        x: wnd.width(),
        y: wnd.height(),
    };

    cursor_pos - window_size / 2.0
}