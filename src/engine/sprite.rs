use super::vectors::{Vector2f, Vector3f};

pub struct Sprite {
    pub pos: Vector3f,
    pub value: u32,
    pub dist: f32,
    pub is_player: bool,
}

#[derive(Default, Copy, Clone)]
pub struct Zorigin {
    pub pos: Vector3f,
    pub dir: Vector2f,
    pub plane: Vector2f,
    pub depth: usize,
    pub rotation: f32,
    pub portal_degree: f32,
}

#[derive(Default, Clone)]
pub struct Zdist {
    pub dist: f32,
    pub portal_depth: usize,
}

impl Sprite {
    pub fn get_player_pos_from_json(json: &serde_json::Value) -> Vector3f {
        let mut pos = Vector3f::default();

        for sprite in json.as_array().unwrap() {
            let value = sprite["index"].as_u64().unwrap() as u32;
            if value != 0 {
                continue;
            }
            let x = sprite["pos"]["x"].as_f64().unwrap() as f32 + 0.5;
            let y = sprite["pos"]["y"].as_f64().unwrap() as f32 + 0.5;
            let z = sprite["pos"]["z"].as_f64().unwrap() as f32;

            pos = Vector3f::new(x, y, z);
        }
        pos
    }

    pub fn parse_sprites_from_json(json: &serde_json::Value) -> Vec<Sprite> {
        let mut sprites = Vec::new();
        for sprite in json.as_array().unwrap() {
            let value = sprite["index"].as_u64().unwrap() as u32;
            let x = sprite["pos"]["x"].as_f64().unwrap() as f32 + 0.5;
            let y = sprite["pos"]["y"].as_f64().unwrap() as f32 + 0.5;
            let z = sprite["pos"]["z"].as_f64().unwrap() as f32;
            let is_player = value == 0;

            sprites.push(Sprite { pos: Vector3f::new(x, y, z), value: value - 1, dist: 0.0, is_player });
        }
        sprites
    }
}