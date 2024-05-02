use crate::types::Board;
use crate::{
    config::{MAX_HEIGHT, MAX_WIDTH},
    types::CellKind,
};
use bevy::prelude::ResMut;

pub fn spread_temperature(board: &mut ResMut<Board>, x: usize, y: usize) {
    transfer_heat(board, x, y, x as i32, y as i32 + 1);
    transfer_heat(board, x, y, x as i32, y as i32 - 1);
    transfer_heat(board, x, y, x as i32 + 1, y as i32);
    transfer_heat(board, x, y, x as i32 - 1, y as i32);
    if board.0[x][y].temperature > 30 && board.0[x][y].temperature < 50 {
        board.0[x][y].temperature -= 5;
    }
    if board.0[x][y].temperature > -10 && board.0[x][y].temperature < 20 {
        board.0[x][y].temperature += 5;
    }
}

pub fn is_out_of_bounds(x: i32, y: i32) -> bool {
    return x < 0 || y < 0 || x >= MAX_WIDTH as i32 || y >= MAX_HEIGHT as i32;
}

fn transfer_heat(board: &mut ResMut<Board>, x: usize, y: usize, next_x: i32, next_y: i32) -> bool {
    // same square
    if x as i32 == next_x && y as i32 == next_y {
        return false;
    }
    if is_out_of_bounds(next_x, next_y) {
        return false;
    }

    let temperature = board.0[x][y].temperature;
    let next_temperature = board.0[next_x as usize][next_y as usize].temperature;
    let temperature_diff = temperature - next_temperature;
    let heat_transfer = temperature_diff / 4;
    board.0[x][y].temperature -= heat_transfer;
    board.0[next_x as usize][next_y as usize].temperature += heat_transfer;

    return true;
}

pub fn displace_soft(
    board: &mut ResMut<Board>,
    x: usize,
    y: usize,
    next_x: i32,
    next_y: i32,
) -> bool {
    // same square
    if x as i32 == next_x && y as i32 == next_y {
        return false;
    }

    // next square is out of bounds
    if is_out_of_bounds(next_x, next_y) {
        return false;
    }

    let cell_weight = board.0[x as usize][y as usize].kind.get_weight();
    let cell_kind = board.0[x as usize][y as usize].kind;

    let next_x = next_x as usize;
    let next_y = next_y as usize;
    let x = x as usize;
    let y = y as usize;

    let next_kind = board.0[next_x][next_y].kind;
    if next_kind.get_is_fixed() {
        return false;
    }

    let next_weight = next_kind.get_weight();
    let next_empty = next_kind == CellKind::Nothing;
    let next_lighter = next_weight < cell_weight;
    if next_empty || next_lighter {
        board.0[next_x][next_y].updated = true;
        board.0[next_x][next_y].kind = cell_kind;
        board.0[x][y].updated = true;
        board.0[x][y].kind = next_kind;
        return true;
    }
    return false;
}
