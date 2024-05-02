use bevy::prelude::{Component, Resource, Timer};

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum CellKind {
    Nothing,
    Sand,
    Water,
    Wood,
    CarbonSmoke,
    Steam,
}

impl CellKind {
    pub fn get_texture(&self) -> String {
        match self {
            CellKind::Nothing => "sprites/particles/nothing.png".to_string(),
            CellKind::Sand => "sprites/particles/sand.png".to_string(),
            CellKind::Water => "sprites/particles/water.png".to_string(),
            CellKind::Wood => "sprites/particles/wood.png".to_string(),
            CellKind::CarbonSmoke => "sprites/particles/carbon_smoke.png".to_string(),
            CellKind::Steam => "sprites/particles/vapor.png".to_string(),
        }
    }
    // approximation in kg/m^3
    pub fn get_weight(&self) -> i32 {
        match self {
            CellKind::Nothing => 0,
            CellKind::Sand => 1500,
            CellKind::Water => 1000,
            CellKind::Wood => 800,
            CellKind::CarbonSmoke => 25,
            CellKind::Steam => 800,
        }
    }

    pub fn get_is_fixed(&self) -> bool {
        match self {
            CellKind::Nothing => false,
            CellKind::Sand => false,
            CellKind::Water => false,
            CellKind::Wood => true,
            CellKind::CarbonSmoke => false,
            CellKind::Steam => false,
        }
    }
    pub fn is_flammable(&self) -> bool {
        match self {
            CellKind::Nothing => false,
            CellKind::Sand => false,
            CellKind::Water => false,
            CellKind::Wood => true,
            CellKind::CarbonSmoke => false,
            CellKind::Steam => false,
        }
    }
}

pub struct Cell {
    pub kind: CellKind,
    pub updated: bool,
    pub temperature: i32,
    pub hp: u8,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            kind: CellKind::Nothing,
            updated: false,
            temperature: 20,
            hp: 100,
        }
    }
}

#[derive(Resource)]
pub struct Board(pub Vec<Vec<Cell>>);

#[derive(Component, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub layer: i32,
}

#[derive(Resource)]
pub struct FrameTimer(pub Timer);
