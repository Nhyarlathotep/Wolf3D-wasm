use super::vectors::Vector3f;
use super::direction::Direction;

use crate::graphics::HSLColor;

pub struct Portal {
    pub pos: Vector3f,
    pub dir: Direction,
    pub hsl: HSLColor,
}

impl Portal {
    pub fn from_json(json: &serde_json::Value) -> Option<Portal> {
        if let Some(portal) = json.as_object() {
            let x = portal["pos"]["x"].as_u64().unwrap() as f32;
            let y = portal["pos"]["y"].as_u64().unwrap() as f32;
            let z = portal["pos"]["z"].as_u64().unwrap() as f32;
            let hue = portal["hue"].as_u64().unwrap() as f32;
            let dir = Direction::from_str(portal["direction"].as_str().unwrap());

            return Some(Portal { pos: Vector3f::new(x, y, z), dir, hsl: HSLColor::new(hue, 0.0, 0.0) });
        }
        None
    }

    pub fn link_dir(&self, rhs: &Portal) -> f32 {
        -(std::f32::consts::PI / 2.0) * (2 - (self.dir as i32 - rhs.dir as i32)) as f32
    }

    pub fn link_x(&self, rhs: &Portal, camera_pos: &Vector3f) -> f32 {
        self.pos.x + match self.dir {
            Direction::North => {
                match rhs.dir {
                    Direction::North => rhs.pos.x + 1.0 - camera_pos.x,
                    Direction::West => rhs.pos.y + 1.0 - camera_pos.y,
                    Direction::South => camera_pos.x - rhs.pos.x,
                    Direction::East => camera_pos.y - rhs.pos.y,
                    _ => 0.0
                }
            }
            Direction::West => {
                1.0 + match rhs.dir {
                    Direction::North => camera_pos.y - rhs.pos.y,
                    Direction::West => rhs.pos.x + 1.0 - camera_pos.x,
                    Direction::South => rhs.pos.y + 1.0 - camera_pos.y,
                    Direction::East => camera_pos.x - rhs.pos.x,
                    _ => 0.0
                }
            }
            Direction::South => {
                match rhs.dir {
                    Direction::North => camera_pos.x - rhs.pos.x,
                    Direction::West => camera_pos.y - rhs.pos.y,
                    Direction::South => rhs.pos.x + 1.0 - camera_pos.x,
                    Direction::East => rhs.pos.y + 1.0 - camera_pos.y,
                    _ => 0.0
                }
            }
            Direction::East => {
                -1.0 + match rhs.dir {
                    Direction::North => rhs.pos.y + 1.0 - camera_pos.y,
                    Direction::West => camera_pos.x - rhs.pos.x,
                    Direction::South => camera_pos.y - rhs.pos.y,
                    Direction::East => rhs.pos.x + 1.0 - camera_pos.x,
                    _ => 0.0
                }
            }
            _ => 0.0
        }
    }

    pub fn link_y(&self, rhs: &Portal, camera_pos: &Vector3f) -> f32 {
        self.pos.y + match self.dir {
            Direction::North => {
                -1.0 + match rhs.dir {
                    Direction::North => rhs.pos.y + 1.0 - camera_pos.y,
                    Direction::West => camera_pos.x - rhs.pos.x,
                    Direction::South => camera_pos.y - rhs.pos.y,
                    Direction::East => rhs.pos.x + 1.0 - camera_pos.x,
                    _ => 0.0
                }
            }
            Direction::West => {
                match rhs.dir {
                    Direction::North => rhs.pos.x + 1.0 - camera_pos.x,
                    Direction::West => rhs.pos.y + 1.0 - camera_pos.y,
                    Direction::South => camera_pos.x - rhs.pos.x,
                    Direction::East => camera_pos.y - rhs.pos.y,
                    _ => 0.0
                }
            }
            Direction::South => {
                1.0 + match rhs.dir {
                    Direction::North => camera_pos.y - rhs.pos.y,
                    Direction::West => rhs.pos.x + 1.0 - camera_pos.x,
                    Direction::South => rhs.pos.y + 1.0 - camera_pos.y,
                    Direction::East => camera_pos.x - rhs.pos.x,
                    _ => 0.0
                }
            }
            Direction::East => {
                match rhs.dir {
                    Direction::North => camera_pos.x - rhs.pos.x,
                    Direction::West => camera_pos.y - rhs.pos.y,
                    Direction::South => rhs.pos.x + 1.0 - camera_pos.x,
                    Direction::East => rhs.pos.y + 1.0 - camera_pos.y,
                    _ => 0.0
                }
            }
            _ => 0.0
        }
    }
}