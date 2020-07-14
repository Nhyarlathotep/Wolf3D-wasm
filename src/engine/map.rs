use super::vectors::Vector3f;
use super::direction::Direction;
use super::cell::{Cell, Interaction};
use super::portal::Portal;

type Cells = Vec<Vec<Vec<Cell>>>;
type Portals = Vec<(Option<Portal>, Option<Portal>)>;

pub struct Map {
    cells: Cells,
    portals: Portals,
    empty_ref: Cell,
}

impl Map {
    fn parse_cells_from_json(json: &serde_json::Value) -> Cells {
        let mut cells = Cells::new();

        for list in json.as_array().unwrap() {
            let mut floor: Vec<Vec<Cell>> = (0..100).map(|_| {
                (0..100).map(|_| {
                    Cell::default()
                }).collect()
            }).collect();

            for cell in list.as_array().unwrap() {
                let x = cell["pos"]["x"].as_u64().unwrap() as usize;
                let y = cell["pos"]["y"].as_u64().unwrap() as usize;

                floor[x][y] = Cell::from_json(cell);
            }
            cells.push(floor);
        }
        cells
    }

    fn parse_portals_from_json(json: &serde_json::Value) -> Portals {
        let mut portals = Portals::new();

        if let Some(list) = json.as_array() {
            for portal in list {
                portals.push((Portal::from_json(&portal["first"]), Portal::from_json(&portal["second"])));
            }
        }
        portals
    }

    pub fn new(map: &serde_json::Value) -> Map {
        Map {
            cells: Map::parse_cells_from_json(&map["cells"]),
            portals: Map::parse_portals_from_json(&map["portals"]),
            empty_ref: Cell::Empty,
        }
    }

    pub fn depth(&self) -> usize {
        self.cells.len()
    }

    pub fn update(&mut self, delta: f32) {
        for floor in &mut self.cells {
            for row in floor {
                for cell in row {
                    cell.update(delta);
                }
            }
        }
    }

    pub fn get(&self, position: &Vector3f) -> &Cell {
        if position.z < 0.0 || position.z as usize > self.depth() {
            return &self.empty_ref;
        }
        if let Some(floor) = self.cells.get(position.z as usize) {
            if let Some(line) = floor.get(position.x as usize) {
                if let Some(cell) = line.get(position.y as usize) {
                    return cell;
                }
            }
        }
        &self.empty_ref
    }

    pub fn get_mut(&mut self, position: &Vector3f) -> &mut Cell {
        if position.z < 0.0 || position.z as usize > self.cells.len() - 1 {
            return &mut self.empty_ref;
        }
        if let Some(floor) = self.cells.get_mut(position.z as usize) {
            if let Some(line) = floor.get_mut(position.x as usize) {
                if let Some(cell) = line.get_mut(position.y as usize) {
                    return cell;
                }
            }
        }
        &mut self.empty_ref
    }

    pub fn portals_at(&self, position: Vector3f, dir: Direction) -> Option<(Option<&Portal>, Option<&Portal>)> {
        for (first_portal, second_portal) in &self.portals {
            if dir == Direction::None && (first_portal.is_none() || second_portal.is_none()) {
                continue;
            }
            if let Some(portal) = first_portal.as_ref() {
                if position == portal.pos && (dir == Direction::None || dir == portal.dir) {
                    return Some((Some(portal), second_portal.as_ref()));
                }
            }
            if let Some(portal) = second_portal.as_ref() {
                if position == portal.pos && (dir == Direction::None || dir == portal.dir) {
                    return Some((Some(portal), first_portal.as_ref()));
                }
            }
        }
        None
    }
}
