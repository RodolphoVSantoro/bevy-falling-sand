use crate::{
    config::{MAX_HEIGHT, MAX_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH},
    types::{Board, Cell, CellKind, Position},
};

use bevy::prelude::{
    AssetServer, Camera2dBundle, Commands, Query, Res, SpriteBundle, Transform, Vec3, Window,
};

pub fn create_board() -> Board {
    let mut board: Board = Board(vec![]);
    for _ in 0..MAX_WIDTH {
        let mut column: Vec<Cell> = vec![];
        for _ in 0..MAX_HEIGHT {
            column.push(Cell::default());
        }
        board.0.push(column);
    }

    return board;
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    //init camera
    commands.spawn(Camera2dBundle::default());
    // init board sprites
    for x in 0..MAX_WIDTH {
        for y in 0..MAX_HEIGHT {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(CellKind::NOTHING.get_texture()),
                    transform: Transform::from_translation(Vec3::new(
                        (x * 10) as f32 - 500.0,
                        (y * 10) as f32 - 300.0,
                        0.,
                    )),
                    ..Default::default()
                },
                Position { x, y },
            ));
        }
    }
    commands.spawn(SpriteBundle {
        texture: asset_server.load("text/sand.png"),
        transform: Transform::from_scale(Vec3::new(0.5, 0.5, 0.5))
            .with_translation(Vec3::new(550.0, 120.0, 0.0)),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("text/water.png"),
        transform: Transform::from_scale(Vec3::new(0.5, 0.5, 0.5))
            .with_translation(Vec3::new(550.0, 0.0, 0.0)),
        ..Default::default()
    });

    let mut window = windows.single_mut();

    window.resolution.set(WINDOW_WIDTH, WINDOW_HEIGHT);
}
