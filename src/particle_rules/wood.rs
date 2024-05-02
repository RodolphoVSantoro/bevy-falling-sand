use crate::types::Board;
use bevy::prelude::ResMut;

pub fn update(board: &mut ResMut<Board>, x: usize, y: usize) {
    if board.0[x][y].temperature < 500 {
        return;
    }
    if board.0[x][y].hp > 10 {
        board.0[x][y].hp -= 10;
    } else {
        board.0[x][y].hp = 0;
    }
    let directions = vec![
        (x as i32, y as i32 - 1),
        (x as i32 + 1, y as i32 - 1),
        (x as i32 - 1, y as i32 - 1),
        (x as i32 - 1, y as i32),
        (x as i32 + 1, y as i32),
        (x as i32 - 1, y as i32 + 1),
        (x as i32, y as i32 + 1),
        (x as i32 + 1, y as i32 + 1),
    ];
    // try to generate smoke in a direction
    // if it fails, try the next direction
    for (dx, dy) in directions {
        if generate_smoke(board, dx, dy) {
            break;
        }
    }
}

fn generate_smoke(board: &mut ResMut<Board>, x: i32, y: i32) -> bool {
    if super::particle::is_out_of_bounds(x, y) {
        return false;
    }
    if board.0[x as usize][y as usize].kind != crate::types::CellKind::Nothing {
        return false;
    }
    board.0[x as usize][y as usize].kind = crate::types::CellKind::CarbonSmoke;
    board.0[x as usize][y as usize].hp = 100;
    return true;
}
