use bevy::ecs::system::ResMut;

mod carbon_smoke;
mod particle;
mod sand;
mod water;
mod wood;

use crate::types::{Board, CellKind};

pub fn update(board: &mut ResMut<Board>, x: usize, y: usize) {
    match board.0[x][y].kind {
        CellKind::Sand => sand::update(board, x, y),
        CellKind::Water => water::update(board, x, y),
        CellKind::Wood => wood::update(board, x, y),
        _ => (),
    }
    particle::spread_temperature(board, x, y);

    if board.0[x][y].hp == 0 {
        board.0[x][y].kind = CellKind::Nothing;
        board.0[x][y].hp = 100;
    }
}
