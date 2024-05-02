use crate::rules::update;

use crate::{
    config::{MAX_HEIGHT, MAX_WIDTH},
    types::{Board, FrameTimer, Position},
};

use bevy::prelude::{Query, Res, ResMut, Sprite, Time};

/**
 * Returns a range that is limited to the positive range of \[0, max\]
 */
// fn limited_positive_range(start: i32, end: i32, max: usize) -> std::ops::Range<usize> {
//     return (cmp::max(start, 0) as usize)..(cmp::min(end + 1, max as i32) as usize);
// }

fn update_cells(board: &mut ResMut<Board>) {
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            update(board, x, y);
        }
    }
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            board.0[x][y].updated = false;
        }
    }
}

fn update_sprites(board: &mut ResMut<Board>, sprite_query: &mut Query<(&mut Sprite, &Position)>) {
    for (mut sprite, position) in sprite_query.iter_mut() {
        let cell = &board.0[position.x][position.y];
        sprite.color = cell.kind.get_color();
    }
}

pub fn process_tick(
    time: Res<Time>,
    mut timer: ResMut<FrameTimer>,
    mut board: ResMut<Board>,
    mut sprite_query: Query<(&mut Sprite, &Position)>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    update_cells(&mut board);
    update_sprites(&mut board, &mut sprite_query);
}
