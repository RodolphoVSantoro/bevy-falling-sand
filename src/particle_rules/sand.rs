use crate::types::Board;
use bevy::prelude::ResMut;

pub fn update(board: &mut ResMut<Board>, x: usize, y: usize) {
    if board.0[x][y].updated {
        return;
    }
    super::particle::displace_soft(board, x, y, x as i32, y as i32 - 1);
    super::particle::displace_soft(board, x, y, x as i32 - 1, y as i32 - 1);
    super::particle::displace_soft(board, x, y, x as i32 + 1, y as i32 - 1);
}
