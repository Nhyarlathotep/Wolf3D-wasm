use super::Direction;
use super::{Interaction, Thin};

#[derive(Clone)]
pub struct ThinWall {
    value: u32,
    depth: f32,
    dir: Direction,
    delta: f32,
    moving: bool,
    pushable: bool,
}

impl ThinWall {
    pub fn new(value: u32, dir: Direction, pushable: bool) -> ThinWall {
        let depth = if pushable { 1.0 } else { 0.5 };

        ThinWall {
            value,
            depth,
            dir,
            delta: 0.0,
            moving: false,
            pushable,
        }
    }
}

impl Interaction for ThinWall {
    fn trigger(&mut self) {
        if self.pushable {
            self.moving = true;
            self.delta = 2.0 * (1.0 - self.depth);
        }
    }

    fn update(&mut self, delta: f32) {
        if !self.pushable || !self.moving {
            return;
        }

        self.delta += delta;
        self.depth = 1.0 - self.delta / 2.0;
        if self.delta >= 2.0 {
            self.depth = 0.0;
            self.moving = false;
        }
    }
}

impl Thin for ThinWall {
    fn value(&self) -> u32 {
        self.value
    }

    fn dir(&self) -> Direction {
        self.dir
    }

    fn slide(&self) -> f32 {
        1.0
    }

    fn depth(&self) -> f32 {
        self.depth
    }

    fn pushable(&self) -> bool {
        self.pushable
    }
}
