use crate::particle_rules::update;

use crate::{
    config::{MAX_HEIGHT, MAX_WIDTH},
    types::{Board, FrameTimer, Position},
};

use bevy::prelude::{AssetServer, Commands, Handle, Image, Query, Res, ResMut, Time};

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

fn update_sprites(
    commands: &mut Commands,
    board: &mut ResMut<Board>,
    asset_server: Res<AssetServer>,
    sprite_query: &mut Query<(&Position, &mut Handle<Image>)>,
) {
    for entity in sprite_query.iter_mut() {
        let (position, mut sprite) = entity;
        if position.layer == 0 {
            let cell = &board.0[position.x][position.y];
            *sprite = asset_server.load(cell.kind.get_texture());
        } else if position.layer == 1 {
            if board.0[position.x][position.y].temperature < 500 {}
        }
    }
}

pub fn process_tick(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<FrameTimer>,
    mut board: ResMut<Board>,
    asset_server: Res<AssetServer>,
    mut sprite_query: Query<(&Position, &mut Handle<Image>)>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    update_cells(&mut board);
    update_sprites(&mut commands, &mut board, asset_server, &mut sprite_query);
}
