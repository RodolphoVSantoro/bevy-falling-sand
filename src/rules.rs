use bevy::ecs::system::ResMut;

use crate::{
    config::{MAX_HEIGHT, MAX_WIDTH},
    types::{Board, CellKind},
};

pub fn update(board: &mut ResMut<Board>, x: usize, y: usize) {
    match board.0[x][y].kind {
        CellKind::SAND => update_sand(board, x, y),
        CellKind::WATER => update_water(board, x, y),
        _ => (),
    }
}

fn swap_if_empty_or_lighter(
    board: &mut ResMut<Board>,
    x: i32,
    y: i32,
    next_x: i32,
    next_y: i32,
) -> bool {
    if x == next_x && y == next_y {
        return false;
    }
    if next_x < 0 || next_y < 0 || next_x >= MAX_WIDTH as i32 || next_y >= MAX_HEIGHT as i32 {
        return false;
    }

    let cell_weight = board.0[x as usize][y as usize].kind.get_weight();
    let cell_kind = board.0[x as usize][y as usize].kind;

    let next_x = next_x as usize;
    let next_y = next_y as usize;
    let x = x as usize;
    let y = y as usize;

    let next_kind = board.0[next_x][next_y].kind;
    let next_weight = next_kind.get_weight();
    let next_empty = next_kind == CellKind::NOTHING;
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

fn update_sand(board: &mut ResMut<Board>, x: usize, y: usize) {
    if board.0[x][y].updated {
        return;
    }
    swap_if_empty_or_lighter(board, x as i32, y as i32, x as i32, y as i32 - 1);
    swap_if_empty_or_lighter(board, x as i32, y as i32, x as i32 - 1, y as i32 - 1);
    swap_if_empty_or_lighter(board, x as i32, y as i32, x as i32 + 1, y as i32 - 1);
}

fn update_water(board: &mut ResMut<Board>, x: usize, y: usize) {
    if board.0[x][y].updated {
        return;
    }

    swap_if_empty_or_lighter(board, x as i32, y as i32, x as i32, y as i32 - 1);
    swap_if_empty_or_lighter(board, x as i32, y as i32, x as i32 - 1, y as i32 - 1);
    swap_if_empty_or_lighter(board, x as i32, y as i32, x as i32 + 1, y as i32 - 1);
    swap_if_empty_or_lighter(board, x as i32, y as i32, x as i32 - 1, y as i32);
    swap_if_empty_or_lighter(board, x as i32, y as i32, x as i32 + 1, y as i32);
}
