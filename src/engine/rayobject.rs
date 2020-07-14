use super::direction::Direction;
use super::vectors::{Vector2f, Vector3f};
use super::map::Map;
use super::portal::Portal;
use super::cell::{Cell, Thin, DOOR_VALUE};
use super::player::Player;

static PORTAL_RECURSION_LIMIT: usize = 3;

pub struct Ray {
    pos: Vector3f,
    ray_dir: Vector3f,
    delta: Vector3f,
    step: Vector3f,
    side_dist: Vector3f,
    pub origin: Vector3f,
    pub portal_recursion: usize,
    dir: Direction,
}

pub struct Hit {
    pub value: Option<u32>,
    pub pos: Vector3f,
    pub dist: f32,
    pub dir: Direction,
    pub texture_pos: Vector2f,
}

impl Ray {
    pub fn new(player: &Player, camera_dir: Vector2f) -> Ray {
        let mut origin = player.pos;
        let pos = Vector3f::new(origin.x.floor(), origin.y.floor(), origin.z.round());
        let dir = Vector3f::new(player.dir.x + player.plane.x * camera_dir.x, player.dir.y + player.plane.y * camera_dir.x, 0.5 * camera_dir.y);
        let delta = Vector3f::new((1.0 / dir.x).abs(), (1.0 / dir.y).abs(), (1.0 / dir.z).abs());
        let mut step = Vector3f::default();
        let mut side_dist = Vector3f::default();

        step.x = if dir.x < 0.0 { -1.0 } else { 1.0 };
        step.y = if dir.y < 0.0 { -1.0 } else { 1.0 };
        step.z = if dir.z < 0.0 { 1.0 } else { -1.0 };

        side_dist.x = if dir.x < 0.0 { origin.x - pos.x } else { pos.x + 1.0 - origin.x } * delta.x;
        side_dist.y = if dir.y < 0.0 { origin.y - pos.y } else { pos.y + 1.0 - origin.y } * delta.y;
        side_dist.z = if dir.z < 0.0 { pos.z - origin.z + 0.5 } else { origin.z - pos.z + 0.5 } * delta.z;

        origin.z += 0.5;

        Ray {
            pos,
            ray_dir: dir,
            delta,
            step,
            side_dist,
            origin,
            portal_recursion: 0,
            dir: Direction::default(),
        }
    }

    fn relocate(&mut self, new_origin: Vector3f, new_dir: f32) {
        let old_dir = self.ray_dir.x;

        self.ray_dir.x = self.ray_dir.x * new_dir.cos() - self.ray_dir.y * new_dir.sin();
        self.ray_dir.y = old_dir * new_dir.sin() + self.ray_dir.y * new_dir.cos();
        self.origin = new_origin;
        self.pos = Vector3f::new(self.origin.x.floor(), self.origin.y.floor(), self.origin.z.round());
        self.delta = Vector3f::new((1.0 / self.ray_dir.x).abs(), (1.0 / self.ray_dir.y).abs(), (1.0 / self.ray_dir.z).abs());
        self.side_dist.x = if self.ray_dir.x < 0.0 { self.origin.x - self.pos.x } else { self.pos.x + 1.0 - self.origin.x } * self.delta.x;
        self.side_dist.y = if self.ray_dir.y < 0.0 { self.origin.y - self.pos.y } else { self.pos.y + 1.0 - self.origin.y } * self.delta.y;
        self.side_dist.z = if self.ray_dir.z < 0.0 { self.pos.z - self.origin.z + 0.5 } else { self.origin.z - self.pos.z + 0.5 } * self.delta.z;
        self.step.x = if self.ray_dir.x < 0.0 { -1.0 } else { 1.0 };
        self.step.y = if self.ray_dir.y < 0.0 { -1.0 } else { 1.0 };
        self.origin.z += 0.5;
    }

    pub fn pass_through_portal(&mut self, dest: &Portal, source: &Portal) -> bool {
        if self.portal_recursion >= PORTAL_RECURSION_LIMIT {
            return false;
        }
        self.portal_recursion += 1;

        let new_origin = Vector3f::new(
            dest.link_x(source, &self.origin),
            dest.link_y(source, &self.origin),
            (dest.pos.z - source.pos.z) + self.origin.z - 0.5);

        self.relocate(new_origin, dest.link_dir(source));
        while self.pos.x != dest.pos.x || self.pos.y != dest.pos.y {
            self.grow();
        }
        if self.pos.z != dest.pos.z {
            self.grow();
        }
        self.grow();
        true
    }

    pub fn cast(&mut self, map: &Map) -> Hit {
        let max_dist = 30.0;
        let mut passed_door = false;
        let mut passed_dor_pos = Vector3f::default();
        let mut passed_height = 1.0;
        let mut passed_through = false;
        let mut passed_pos = Vector3f::default();

        loop {
            if passed_through {
                let dist = self.compute_dist();
                let texture_pos = self.compute_texture_pos(&self.dir, dist);

                if (self.dir == Direction::Up || (self.dir.is_side() && texture_pos.y <= passed_height)) && self.is_block_adjacent(passed_pos, self.pos) {
                    let dir = if self.ray_dir.z > 0.0 { Direction::Up } else { Direction::Down };
                    let dist = (passed_pos.z - (1.0 - passed_height) - self.origin.z + (1.0 - self.step.z) / 2.0) / self.ray_dir.z;
                    let texture_pos = self.compute_texture_pos(&dir, dist);
                    return Hit { value: Some(map.get(&passed_pos).value()), pos: self.pos, dist, dir, texture_pos };
                }
            } else {
                passed_through = false;
            }

            if passed_door && (!self.is_block_adjacent(passed_dor_pos, self.pos) || self.pos.z != passed_dor_pos.z) {
                passed_door = false;
            }

            match map.get(&self.pos) {
                Cell::Empty => {
                    if self.pos.z < 0.0 || self.pos.z >= map.depth() as f32 || ((self.pos.x - self.origin.x).powf(2.0) + (self.pos.y - self.origin.y).powf(2.0)).sqrt() > max_dist {
                        let dist = self.compute_dist();

                        return Hit { value: None, pos: self.pos, dist, dir: self.dir, texture_pos: Vector2f::default() };
                    }
                }
                Cell::Wall { value, height } => {
                    let dist = self.compute_dist();
                    let texture_pos = self.compute_texture_pos(&self.dir, dist);

                    passed_pos = self.pos;
                    passed_height = *height;
                    if passed_height != 1.0 {
                        let mut y = texture_pos.y;

                        if y == 0.0 {
                            y = 1.0;
                        }
                        if (y > passed_height && self.dir.is_side()) || (self.dir == Direction::Up) {
                            passed_through = true;
                        } else {
                            return Hit { value: Some(*value), pos: self.pos, dist, dir: self.dir, texture_pos };
                        }
                    } else {
                        let value = if passed_door { DOOR_VALUE + 2 } else { *value };

                        return Hit { value: Some(value), pos: self.pos, dist, dir: self.dir, texture_pos };
                    }
                }
                Cell::Thin(thin) => {
                    if let Some((dist, texture_pos)) = self.grow_thin(thin) {
                        return Hit { value: Some(thin.value()), pos: self.pos, dist, dir: thin.dir(), texture_pos };
                    }
                    if thin.value() >= DOOR_VALUE {
                        passed_door = true;
                        passed_dor_pos = self.pos;
                    }
                }
            }
            self.grow();
        }
    }

    fn grow_x(&mut self) {
        self.pos.x += self.step.x;
        self.side_dist.x += self.delta.x;
        self.dir = if self.origin.x < self.pos.x { Direction::East } else { Direction::West };
    }

    fn grow_y(&mut self) {
        self.pos.y += self.step.y;
        self.side_dist.y += self.delta.y;
        self.dir = if self.origin.y < self.pos.y { Direction::North } else { Direction::South };
    }

    fn grow_z(&mut self) {
        self.pos.z += self.step.z;
        self.side_dist.z += self.delta.z;
        self.dir = if self.ray_dir.z > 0.0 { Direction::Up } else { Direction::Down };
    }

    pub fn grow(&mut self) {
        if self.side_dist.x < self.side_dist.y {
            if self.side_dist.x < self.side_dist.z {
                self.grow_x();
            } else {
                self.grow_z();
            }
        } else {
            if self.side_dist.y < self.side_dist.z {
                self.grow_y();
            } else {
                self.grow_z();
            }
        }
    }

    fn grow_thin(&mut self, cell: &Box<dyn Thin>) -> Option<(f32, Vector2f)> {
        let dir = cell.dir();
        let slide = cell.slide();
        let mut depth = cell.depth();
        let mut pos2 = self.pos;

        if self.origin.x < self.pos.x {
            pos2.x -= 1.0;
        }
        if self.origin.y > self.pos.y {
            pos2.y += 1.0;
        }

        let under_light = dir.is_under_light();
        let limit_x = if dir == Direction::East { depth } else { 1.0 - depth };
        let limit_y = if dir == Direction::North { depth } else { 1.0 - depth };

        let ray_mult = if under_light {
            let facing_north = dir == Direction::North && self.ray_dir.y < 0.0;
            let facing_south = dir == Direction::South && self.ray_dir.y > 0.0;

            if facing_north || facing_south {
                depth = 1.0 - depth;
            }
            let offset = if self.origin.y >= self.pos.y && self.origin.y <= self.pos.y + depth {
                //Player is inside
                let relative_y = self.origin.y - self.origin.y.floor();
                if facing_north && relative_y > depth || facing_south && relative_y > depth {
                    return None;
                }
                if depth > 0.5 && self.step.y < 0.0 {
                    0.0
                } else {
                    -1.0
                }
            } else {
                //Player is outside
                0.0
            };
            (pos2.y - self.origin.y + offset) / self.ray_dir.y
        } else {
            //Handle x facing walls
            let facing_west = dir == Direction::East && self.ray_dir.x < 0.0;
            let facing_east = dir == Direction::West && self.ray_dir.x > 0.0;

            if facing_west || facing_east {
                depth = 1.0 - depth;
            }
            let offset = if self.origin.x >= self.pos.x && self.origin.x <= self.pos.x + depth {
                //Player is inside
                let relative_x = self.origin.x - self.origin.x.floor();
                if facing_west && relative_x > depth || facing_east && relative_x > depth {
                    return None;
                }
                if depth > 0.5 && self.step.x < 0.0 {
                    1.0
                } else {
                    0.0
                }
            } else {
                //Player is outside
                1.0
            };
            (pos2.x - self.origin.x + offset) / self.ray_dir.x
        };

        let ray2 = Vector2f::new(self.origin.x + self.ray_dir.x * ray_mult, self.origin.y + self.ray_dir.y * ray_mult);
        let delta = Vector2f::new((1.0 + self.ray_dir.y.powf(2.0) / self.ray_dir.x.powf(2.0)).sqrt(), (1.0 + self.ray_dir.x.powf(2.0) / self.ray_dir.y.powf(2.0)).sqrt());
        let true_step = Vector2f::new((delta.y.powf(2.0) - 1.0).sqrt(), (delta.x.powf(2.0) - 1.0).sqrt());
        let half_step_in = Vector2f::new(ray2.x + (self.step.x * true_step.x) * depth, ray2.y + (self.step.y * true_step.y) * depth);

        if !under_light {
            if half_step_in.y.floor() == self.pos.y && (self.pos.y - half_step_in.y).abs() <= slide {
                let dist = if self.origin.x < self.pos.x + limit_x {
                    if self.step.x < 0.0 {
                        return None;
                    }
                    (half_step_in.x - self.origin.x + (1.0 - true_step.x) * depth) / self.ray_dir.x
                } else {
                    if self.step.x > 0.0 {
                        return None;
                    }
                    (half_step_in.x - self.origin.x + (true_step.x - 1.0) * depth) / self.ray_dir.x
                };
                let mut texture_pos = Vector2f::new(self.origin.y + dist * self.ray_dir.y - slide, self.origin.z + dist * -self.ray_dir.z);
                if texture_pos.y < self.pos.z || texture_pos.y > self.pos.z + 1.0 {
                    return None;
                }
                texture_pos.x -= texture_pos.x.floor();
                texture_pos.y -= texture_pos.y.floor();
                return Some((dist, texture_pos));
            }
        } else {
            if half_step_in.x.floor() == self.pos.x && (self.pos.x - half_step_in.x).abs() <= slide {
                let dist = if self.origin.y < self.pos.y + limit_y {
                    if self.step.y < 0.0 {
                        return None;
                    }
                    (half_step_in.y - self.origin.y + (1.0 - true_step.y) * depth) / self.ray_dir.y
                } else {
                    if self.step.y > 0.0 {
                        return None;
                    }
                    (half_step_in.y - self.origin.y + (true_step.y - 1.0) * depth) / self.ray_dir.y
                };
                let mut texture_pos = Vector2f::new(self.origin.x + dist * self.ray_dir.x - slide, self.origin.z + dist * -self.ray_dir.z);
                if texture_pos.y < self.pos.z || texture_pos.y > self.pos.z + 1.0 {
                    return None;
                }
                texture_pos.x -= texture_pos.x.floor();
                texture_pos.y -= texture_pos.y.floor();
                return Some((dist, texture_pos));
            }
        }
        None
    }

    fn compute_dist(&self) -> f32 {
        match self.dir {
            Direction::None => 0.0,
            Direction::North | Direction::South => {
                (self.pos.y - self.origin.y + (1.0 - self.step.y) / 2.0) / self.ray_dir.y
            }
            Direction::East | Direction::West => {
                (self.pos.x - self.origin.x + (1.0 - self.step.x) / 2.0) / self.ray_dir.x
            }
            Direction::Up | Direction::Down => {
                (self.pos.z - self.origin.z + (1.0 - self.step.z) / 2.0) / self.ray_dir.z
            }
        }
    }

    fn compute_texture_pos(&self, dir: &Direction, dist: f32) -> Vector2f {
        let mut texture_pos = match dir {
            Direction::None => Vector2f::default(),
            Direction::North | Direction::South => {
                Vector2f::new(self.origin.x + dist * self.ray_dir.x, self.origin.z + dist * -self.ray_dir.z)
            }
            Direction::West | Direction::East => {
                Vector2f::new(self.origin.y + dist * self.ray_dir.y, self.origin.z + dist * -self.ray_dir.z)
            }
            Direction::Up | Direction::Down => {
                Vector2f::new(
                    ((self.pos.x - self.origin.x + (1.0 - self.step.x) / 2.0) / self.ray_dir.x + dist) * self.ray_dir.x,
                    ((self.pos.y - self.origin.y + (1.0 - self.step.y) / 2.0) / self.ray_dir.y + dist) * self.ray_dir.y)
            }
        };
        texture_pos.x -= texture_pos.x.floor();
        texture_pos.y -= texture_pos.y.floor();
        texture_pos
    }

    fn is_block_adjacent(&self, b1: Vector3f, b2: Vector3f) -> bool {
        (b1.x == b2.x && b1.y == b2.y && (b1.z - 1.0 == b2.z || b1.z + 1.0 == b2.z)) ||
            (b1.x == b2.x && (b1.y - 1.0 == b2.y || b1.y + 1.0 == b2.y) && b1.z == b2.z) ||
            ((b1.x - 1.0 == b2.x || b1.x + 1.0 == b2.x) && b1.y == b2.y && b1.z == b2.z)
    }
}