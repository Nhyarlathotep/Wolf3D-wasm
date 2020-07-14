use super::Direction;
use super::{Interaction, Thin};

#[derive(PartialEq)]
enum DoorState {
    Closed,
    Opened,
    Closing,
    Opening,
}

pub struct Door {
    value: u32,
    slide: f32,
    dir: Direction,
    delta: f32,
    state: DoorState,
}

impl Door {
    pub fn new(value: u32, dir: Direction) -> Door {
        Door {
            value,
            slide: 1.0,
            dir,
            delta: 0.0,
            state: DoorState::Closed,
        }
    }
}

impl Interaction for Door {
    fn trigger(&mut self) {
        if self.state == DoorState::Closed {
            self.delta = 0.0;
            self.state = DoorState::Opening
        }
    }

    fn update(&mut self, delta: f32) {
        match self.state {
            DoorState::Opened => {
                self.delta += delta;

                if self.delta >= 3.0 {
                    self.delta = 0.0;
                    self.state = DoorState::Closing;
                }
            }
            DoorState::Closed => {}
            DoorState::Opening => {
                self.slide = 1.0 - self.delta / 1.5;
                self.delta += delta;
                if self.slide < 0.0 {
                    self.delta = 0.0;
                    self.state = DoorState::Opened;
                }
            }
            DoorState::Closing => {
                self.slide = self.delta / 1.5;
                self.delta += delta;
                if self.slide > 1.0 {
                    self.delta = 0.0;
                    self.slide = 1.0;
                    self.state = DoorState::Closed;
                }
            }
        }
    }
}

impl Thin for Door {
    fn value(&self) -> u32 {
        self.value
    }

    fn dir(&self) -> Direction {
        self.dir
    }

    fn slide(&self) -> f32 {
        self.slide
    }

    fn depth(&self) -> f32 {
        0.5
    }

    fn pushable(&self) -> bool {
        false
    }
}


