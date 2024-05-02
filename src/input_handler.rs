use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{EventReader, Query, ResMut, Window, With};
use bevy::window::PrimaryWindow;

use crate::types::{Board, CellKind};

pub fn mouse_button_events(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut board: ResMut<Board>,
) {
    static mut SELECTED_KIND: CellKind = CellKind::Sand;
    static mut MOUSE_HELD_BOARD: bool = false;

    let (x, y) = if let Some(position) = q_windows.single().cursor_position() {
        (position.x - 100.0, position.y - 100.0)
    } else {
        return;
    };
    let inside_sand_text = x >= 1000.0 && x < 1100.0 && y >= 130.0 && y < 230.0;
    let inside_water_text = x >= 1000.0 && x < 1100.0 && y >= 250.0 && y < 350.0;
    let inside_wood_text = x >= 1000.0 && x < 1100.0 && y >= 370.0 && y < 470.0;

    let inside_board = x >= 0.0 && x < 1000.0 && y >= 0.0 && y < 600.0;
    let is_mouse_held = unsafe { MOUSE_HELD_BOARD };

    if !inside_board && is_mouse_held {
        unsafe {
            MOUSE_HELD_BOARD = false;
        }
    }

    for ev in mousebtn_evr.read() {
        match ev.state {
            ButtonState::Pressed => {
                if inside_board {
                    unsafe {
                        MOUSE_HELD_BOARD = true;
                    }
                } else if inside_sand_text {
                    unsafe {
                        SELECTED_KIND = CellKind::Sand;
                    }
                } else if inside_water_text {
                    unsafe {
                        SELECTED_KIND = CellKind::Water;
                    }
                } else if inside_wood_text {
                    unsafe {
                        SELECTED_KIND = CellKind::Wood;
                    }
                }
            }

            ButtonState::Released => unsafe {
                MOUSE_HELD_BOARD = false;
            },
        }
    }

    let is_mouse_held = unsafe { MOUSE_HELD_BOARD };
    if is_mouse_held {
        let x = (x / 10.0).floor();
        let y = 59.0 - (y / 10.0).floor();
        unsafe {
            board.0[x as usize][y as usize].kind = SELECTED_KIND;
        }
    }
}
