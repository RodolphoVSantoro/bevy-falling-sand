use bevy::{
    prelude::{Component, Resource, Timer},
    render::color::Color,
};

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum CellKind {
    NOTHING = 0,
    SAND = 1,
    WATER = 2,
}

const NOTHING_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const SAND_COLOR: Color = Color::rgb(0.89, 0.70, 0.02);
const WATER_COLOR: Color = Color::rgb(0.02, 0.02, 0.89);

impl CellKind {
    pub fn get_color(&self) -> Color {
        match self {
            CellKind::NOTHING => NOTHING_COLOR,
            CellKind::SAND => SAND_COLOR,
            CellKind::WATER => WATER_COLOR,
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
