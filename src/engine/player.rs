use super::map::Map;
use super::cell::{Cell, Interaction};
use super::vectors::{Vector2f, Vector3f};
use crate::engine::Direction;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Action {
    None = 0,
    MoveForward = 90,
    MoveForward2 = 87,
    MoveBackward = 83,
    LookLeft = 81,
    LookLeft2 = 65,
    LookRight = 68,
    Jump = 32,
    Interact = 70,
}

pub struct Player {
    pub pos: Vector3f,
    pub dir: Vector2f,
    pub plane: Vector2f,
    pub frame: u32,

    delta: f32,
    gravity: f32,
    velocity: Vector2f,
    rotation: Vector2f,
    action: Action,
}

impl Player {
    pub fn new(pos: Vector3f) -> Player {
        let mut player = Player {
            pos,
            dir: Vector2f::new( 1.0, 0.0),
            plane: Vector2f::new(0.0, 0.66),
            frame: 0,
            delta: 0.0,
            gravity: -3.8,
            velocity: Vector2f::default(),
            rotation: Vector2f::default(),
            action: Action::None,
        };
        player.update_dir(std::f32::consts::PI / 2.0, 1.0);
        player
    }

    pub fn handle_inputs(&mut self, key: u32, pressed: bool) {
        if key == Action::MoveForward as u32 || key == Action::MoveForward2 as u32 {
            self.velocity.x = if pressed { 4.0 } else { 0.0 }
        } else if key == Action::MoveBackward as u32 {
            self.velocity.x = if pressed { -4.0 } else { 0.0 }
        } else if key == Action::Jump as u32 {
            if pressed && self.velocity.y == 0.0 {
                self.velocity.y = 1.65;
            }
        } else if key == Action::LookLeft as u32 || key == Action::LookLeft2 as u32 {
            self.rotation.x = if pressed { -3.5 } else { 0.0 }
        } else if key == Action::LookRight as u32 {
            self.rotation.x = if pressed { 3.5 } else { 0.0 }
        } else if key == Action::Interact as u32 {
            self.action = Action::Interact;
        }
    }

    fn update_gravity(&mut self, map: &Map, delta: f32) {
        let mut future_z = self.pos.z + self.velocity.y * delta;
        let inside_wall = map.get(&self.pos);
        let future_under_wall = map.get(&Vector3f::new(self.pos.x, self.pos.y, future_z));

        self.velocity.y += self.gravity * delta;
        self.pos.z = if future_z < 0.0 {
            self.velocity.y = 0.0;
            0.0
        } else {
            if let Cell::Wall {value: _, height} = inside_wall {
                if future_z <= self.pos.z.floor() + *height {
                    future_z = self.pos.z.floor() + *height;
                    self.velocity.y = 0.0;
                }
            } else if let Cell::Empty = inside_wall {
                if future_z <= future_z.floor() + future_under_wall.height() {
                    future_z = future_z.floor() + future_under_wall.height();
                    self.velocity.y = 0.0;
                }
            }
            future_z
        }
    }

    fn move_x_in_thin_wall(&mut self, x_coord: f32, new_x: f32, dir: Direction, slide: f32, depth: f32) {
        if dir.is_under_light() {
            self.pos.x = new_x;
        } else {
            let limit = x_coord + if dir == Direction::East { depth } else { 1.0 - depth };
            let relative_pos_y = self.pos.y - self.pos.y.floor();

            if (self.pos.x > limit && new_x < limit || self.pos.x < limit && new_x > limit) && relative_pos_y < slide {
                return;
            }
            self.pos.x = new_x;
        }
    }

    fn move_y_in_thin_wall(&mut self, y_coord: f32, new_y: f32, dir: Direction, slide: f32, depth: f32) {
        if !dir.is_under_light() {
            self.pos.y = new_y;
        } else {
            let limit = y_coord + if dir == Direction::North { depth } else { 1.0 - depth };
            let relative_pos_x = self.pos.x - self.pos.x.floor();

            if (self.pos.y > limit && new_y < limit || self.pos.y < limit && new_y > limit) && relative_pos_x < slide {
                return;
            }
            self.pos.y = new_y;
        }
    }

    fn update_pos(&mut self, map: &mut Map, delta: f32) {
        let speed = self.velocity.x * delta;
        let new_x = self.pos.x + self.dir.x * speed;
        let new_y = self.pos.y + self.dir.y * speed;

        match map.portals_at(Vector3f::new(new_x.floor(), new_y.floor(), self.pos.z.floor()), Direction::None) {
            None => {
                match map.get(&Vector3f::new(new_x, self.pos.y, self.pos.z)) {
                    Cell::Empty => {
                        match map.get(&self.pos) {
                            Cell::Thin(thin) => self.move_x_in_thin_wall(self.pos.x.floor(), new_x, thin.dir(), thin.slide(), thin.depth()),
                            _ => self.pos.x = new_x
                        }
                    }
                    Cell::Wall { value: _, height } => {
                        if self.pos.z >= self.pos.z.floor() + *height {
                            self.pos.x = new_x
                        }
                    }
                    Cell::Thin(thin) => self.move_x_in_thin_wall(new_x.floor(), new_x, thin.dir(), thin.slide(), thin.depth())
                }

                match map.get(&Vector3f::new(self.pos.x, new_y, self.pos.z)) {
                    Cell::Empty => {
                        match map.get(&self.pos) {
                            Cell::Thin(thin) => self.move_y_in_thin_wall(self.pos.y.floor(), new_y, thin.dir(), thin.slide(), thin.depth()),
                            _ => self.pos.y = new_y
                        }
                    }
                    Cell::Wall { value: _, height } => {
                        if self.pos.z >= self.pos.z.floor() + *height {
                            self.pos.y = new_y
                        }
                    }
                    Cell::Thin(thin) => self.move_y_in_thin_wall(new_y.floor(), new_y, thin.dir(), thin.slide(), thin.depth())
                }
            }
            Some((first, second)) => {
                let source = first.unwrap();
                let dest = second.unwrap();

                let tmp = self.pos;
                self.update_dir(dest.link_dir(source), 1.0);
                self.pos.x = dest.link_x(source, &tmp) + self.dir.x * speed;
                self.pos.y = dest.link_y(source, &tmp) + self.dir.y * speed;
                self.pos.z = dest.pos.z;
            }
        }
    }

    fn update_dir(&mut self, mut new_rotation: f32, delta: f32) {
        new_rotation *= delta;
        self.dir.rotate(new_rotation);
        self.plane.rotate(new_rotation);
    }

    pub fn update(&mut self, map: &mut Map, delta: f32) {
        if self.action == Action::Interact {
            let hit = crate::engine::rayobject::Ray::new(&self, Vector2f::default()).cast(map);

            if let Some(_) = hit.value {
                if hit.dist <= 1.5 {
                    map.get_mut(&hit.pos).trigger();
                }
            }
            self.action = Action::None;
        }
        if self.velocity.y != 0.0 || self.pos.z > 0.0 {
            self.update_gravity(map, delta);
        }
        if self.velocity.x != 0.0 {
            self.update_pos(map, delta);
            self.delta += delta;
            if self.delta > 0.16 {
                self.frame -= 1;
                self.frame = (self.frame + 1) % 4 + 1;
                self.delta = 0.0;
            }
        } else {
            self.frame = 0;
        }
        if self.rotation.x != 0.0 {
            self.update_dir(self.rotation.x, delta);
        }
    }
}