use bevy::prelude::{Component, Resource, Timer};

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum CellKind {
    NOTHING = 0,
    SAND = 1,
    WATER = 2,
}

impl CellKind {
    pub fn get_texture(&self) -> String {
        match self {
            CellKind::NOTHING => "sprites/particles/nothing.png".to_string(),
            CellKind::SAND => "sprites/particles/sand.png".to_string(),
            CellKind::WATER => "sprites/particles/water.png".to_string(),
        }
    }
    pub fn get_weight(&self) -> i32 {
        match self {
            CellKind::NOTHING => 0,
            CellKind::SAND => 1500,
            CellKind::WATER => 1000,
        }
    }
}

pub struct Cell {
    pub kind: CellKind,
    pub updated: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            kind: CellKind::NOTHING,
            updated: false,
        }
    }
}

#[derive(Resource)]
pub struct Board(pub Vec<Vec<Cell>>);

#[derive(Component, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Resource)]
pub struct FrameTimer(pub Timer);
