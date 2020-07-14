#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum Direction {
    None,
    North,
    West,
    South,
    East,
    Up,
    Down,
}

impl Direction {
    pub fn from_str(str: &str) -> Direction {
        match str {
            "North" | "North/South" => Direction::North,
            "West" | "East/West" => Direction::West,
            "South" => Direction::South,
            "East" => Direction::East,
            _ => Direction::None
        }
    }

    pub fn is_side(&self) -> bool {
        *self < Direction::Up
    }

    pub fn is_under_light(&self) -> bool {
        match self {
            Direction::None => false,
            Direction::North | Direction::South | Direction::Up => true,
            Direction::West | Direction::East | Direction::Down => false
        }
    }

    pub fn to_degree(&self) -> f32 {
        match self {
            Direction::North => 270.0,
            Direction::West => 360.0,
            Direction::South => 90.0,
            Direction::East => 180.0,
            Direction::None | Direction::Up | Direction::Down => 0.0
        }
    }
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::None
    }
}