use door::{Door};
use thinwall::ThinWall;
use super::Direction;

mod door;
mod thinwall;

pub static DOOR_VALUE: u32 = 13;

pub trait Interaction {
    fn trigger(&mut self);

    fn update(&mut self, delta: f32);
}

pub trait Thin: Interaction {
    fn value(&self) -> u32;

    fn dir(&self) -> Direction;

    fn slide(&self) -> f32;

    fn depth(&self) -> f32;

    fn pushable(&self) -> bool;
}

pub enum Cell {
    Empty,
    Wall { value: u32, height: f32 },
    Thin(Box<dyn Thin>),
}

impl Cell {
    pub fn from_json(json: &serde_json::Value) -> Cell {
        let cell = json.as_object().unwrap();
        let value = cell["value"].as_u64().unwrap() as u32;

        if cell.contains_key("thin") {
            let dir = Direction::from_str(cell["direction"].as_str().unwrap());
            let pushable = cell["pushable"].as_bool().unwrap();

            if value >= DOOR_VALUE {
                Cell::Thin(Box::new(Door::new(value, dir)))
            } else {
                Cell::Thin(Box::new(ThinWall::new(value, dir, pushable)))
            }
        } else {
            let height = cell["height"].as_f64().unwrap() as f32;

            Cell::Wall { value, height }
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Cell::Empty => 0,
            Cell::Wall { value, .. } => *value,
            Cell::Thin(thin) => thin.value()
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            Cell::Empty => -1.0,
            Cell::Wall { height, .. } => *height,
            Cell::Thin(_) => 1.0
        }
    }
}

impl Interaction for Cell {
    fn trigger(&mut self) {
        if let Cell::Thin(thin) = self {
            thin.trigger();
        }
    }

    fn update(&mut self, delta: f32) {
        if let Cell::Thin(thin) = self {
            thin.update(delta);
            if thin.pushable() && thin.depth() <= 0.0 {
                *self = Cell::Empty;
            }
        }
    }
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::Empty
    }
}