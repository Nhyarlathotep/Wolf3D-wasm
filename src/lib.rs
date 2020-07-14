use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod engine;
mod graphics;

use graphics::{Canvas, RGBColor, HSLColor};
use graphics::color::WHITE;
use graphics::textures;
use engine::{Player, Map};
use engine::vectors::{Vector2f, Vector2i, Vector3f};
use engine::rayobject::{Ray, Hit};
use engine::sprite::{Sprite, Zdist, Zorigin};
use std::cmp::Ordering::{Less, Greater};

extern crate serde_derive;

#[wasm_bindgen()]
pub struct Game {
    map: Map,
    player: Player,
    sprites: Vec<Sprite>,

    canvas: Canvas,
    z_buffer: Vec<Vec<Zdist>>,
    z_origins: Vec<Zorigin>,
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen()]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(map: &JsValue, width: usize, height: usize) -> Game {
        let map = &map.into_serde().unwrap();

        Game {
            map: Map::new(map),
            player: Player::new(Sprite::get_player_pos_from_json(&map["sprites"])),
            sprites: Sprite::parse_sprites_from_json(&map["sprites"]),
            canvas: Canvas::new(width, height),

            z_buffer: vec![vec![Zdist::default(); width]; height],
            z_origins: Vec::new(),
        }
    }

    pub fn process_event(&mut self, key: u32, pressed: bool) {
        self.player.handle_inputs(key, pressed);
    }

    fn compute_pixel(&mut self, x: usize, y: usize, hit: &Hit, ray: &mut Ray) -> RGBColor {
        self.z_buffer[y][x] = Zdist { dist: hit.dist.abs(), portal_depth: ray.portal_recursion };
        match hit.value {
            None => {
                if y > self.canvas.height / 2 {
                    RGBColor::new(113, 113, 113)
                } else {
                    RGBColor::new(56, 56, 56)
                }
            }
            Some(value) => {
                let (text_x, text_y) = textures::get_texture_coord(hit.texture_pos.x, hit.texture_pos.y);

                let mut color = match self.map.portals_at(hit.pos, hit.dir) {
                    None => {
                        textures::get_wall_pixel(text_x + 64 * !hit.dir.is_under_light() as u32, text_y + 64 * value)
                    }
                    Some((source, dest)) => {
                        let portal_color = textures::get_portal_pixel(text_x, text_y);

                        if source.is_some() && dest.is_some() {
                            let dest = dest.unwrap();
                            let source = source.unwrap();

                            if portal_color == WHITE {
                                //Portal's mask
                                textures::get_wall_pixel(text_x + 64 * !hit.dir.is_under_light() as u32, text_y + 64 * value)
                            } else if portal_color.a == 255 {
                                //Portal's border
                                RGBColor::from_hsl(&(HSLColor::from_rgb(&portal_color) + source.hsl))
                            } else {
                                //Portal's center
                                if ray.pass_through_portal(dest, source) {
                                    let depth = ray.portal_recursion;
                                    let mut pos = ray.origin;
                                    let mut dir = self.player.dir;
                                    let mut plane = self.player.plane;
                                    let mut rotation = dest.link_dir(source);
                                    let mut portal_degree = hit.dir.to_degree();

                                    if depth > 1 {
                                        for idx in (0..self.z_origins.len()).rev() {
                                            if self.z_origins[idx].depth == depth - 1 {
                                                rotation += self.z_origins[idx].rotation;
                                                portal_degree = self.z_origins[idx].portal_degree;
                                                break;
                                            }
                                        }
                                    }
                                    pos.z -= 0.5;
                                    dir.rotate(rotation);
                                    plane.rotate(rotation);

                                    let mut find = false;
                                    for idx in (0..self.z_origins.len()).rev() {
                                        if self.z_origins[idx].pos == pos && self.z_origins[idx].depth == depth {
                                            find = true;
                                            break;
                                        }
                                    }
                                    if find == false {
                                        self.z_origins.push(
                                            Zorigin { pos, dir, plane, depth, rotation, portal_degree }
                                        );
                                    }

                                    let new_hit = ray.cast(&mut self.map);
                                    self.compute_pixel(x, y, &new_hit, ray)
                                } else {
                                    RGBColor::from_hsl(&(HSLColor::from_rgb(&portal_color) + source.hsl))
                                }
                            }
                        } else {
                            if portal_color == WHITE {
                                textures::get_wall_pixel(text_x + 64 * !hit.dir.is_under_light() as u32, text_y + 64 * value)
                            } else {
                                RGBColor::from_hsl(&(HSLColor::from_rgb(&portal_color) + source.unwrap().hsl))
                            }
                        }
                    }
                };
                if color.a != 255 {
                    ray.grow();
                    let new_hit = ray.cast(&self.map);

                    color.blend(&self.compute_pixel(x, y, &new_hit, ray));
                }
                color
            }
        }
    }

    fn draw_view(&mut self) {
        let canvas_width = self.canvas.width as f32;
        let canvas_height = self.canvas.height as f32;

        self.z_origins.clear();
        for x in 0..self.canvas.width {
            for y in 0..self.canvas.height {
                let mut ray = Ray::new(&self.player, Vector2f::new(2.0 * x as f32 / canvas_width - 1.0, 2.0 * y as f32 / canvas_height - 1.0));
                let hit = ray.cast(&self.map);
                let color = self.compute_pixel(x, y, &hit, &mut ray);

                self.canvas.put_pixel(x, y, color);
            }
        }
    }

    fn draw_sprites(&mut self) {
        for idx in (0..self.z_origins.len()).rev() {
            let new_origin = self.z_origins[idx];

            self.draw_sprite(new_origin.depth, new_origin.pos, new_origin.dir, new_origin.plane, new_origin.rotation, new_origin.portal_degree);
        }
        self.draw_sprite(0, self.player.pos, self.player.dir, self.player.plane, 0.0, 0.0);
    }

    fn draw_sprite(&mut self, depth: usize, pos: Vector3f, dir: Vector2f, plane: Vector2f, rotation: f32, shift_degree: f32) {
        let canvas_width = self.canvas.width as i32;
        let canvas_height = self.canvas.height as i32;

        for sprite in &mut self.sprites {
            if sprite.is_player {
                sprite.pos = self.player.pos;
            }
            sprite.dist = (pos.x - sprite.pos.x).powf(2.0) + (pos.y - sprite.pos.y).powf(2.0);
        }
        self.sprites.sort_by(|l, r| {
            if l.is_player && l.dist == r.dist {
                Greater
            } else if r.is_player && l.dist == r.dist {
                Less
            } else {
                r.dist.partial_cmp(&l.dist).unwrap()
            }
        });

        for sprite in &self.sprites {
            let relative_pos = Vector2f::new(sprite.pos.x - pos.x, sprite.pos.y - pos.y);
            let transform = Vector2f::new((dir.y * relative_pos.x - dir.x * relative_pos.y) * 1.0 / (plane.x * dir.y - dir.x * plane.y),
                                          (-plane.y * relative_pos.x + plane.x * relative_pos.y) * 1.0 / (plane.x * dir.y - dir.x * plane.y));
            let sprite_canvas_x = ((self.canvas.width / 2) as f32 * (1.0 + transform.x / transform.y)) as i32;
            let sprite_size = (canvas_height as f32 / transform.y).abs() as i32;
            let x_bounds = Vector2i::new(sprite_canvas_x - sprite_size / 2, sprite_size / 2 + sprite_canvas_x).clamp(0, canvas_width, 0, canvas_width);
            let draw_end_y = ((self.canvas.height / 2) as f32 + sprite_size as f32 * (pos.z + 0.5) - sprite_size as f32 * sprite.pos.z) as i32;
            let y_bounds = Vector2i::new(draw_end_y - sprite_size, draw_end_y).clamp(0, canvas_height, 0, canvas_height);

            for x in x_bounds.x..x_bounds.y {
                let text_x = ((x - (sprite_canvas_x - sprite_size / 2)) * 64 / sprite_size) as u32;

                if transform.y > 0.0 && x > 0 && x < canvas_width {
                    let step = 64.0 / sprite_size as f32;
                    let mut text_pos = ((y_bounds.x as f32 - sprite_size as f32 * (pos.z - sprite.pos.z)) - canvas_height as f32 / 2.0 + sprite_size as f32 / 2.0) * step;

                    for y in y_bounds.x..y_bounds.y {
                        let text_y = text_pos as u32 & (64 - 1) as u32;

                        text_pos += step;
                        if (depth != self.z_buffer[y as usize][x as usize].portal_depth && !sprite.is_player) || transform.y > self.z_buffer[y as usize][x as usize].dist {
                            continue;
                        }
                        let color = if sprite.is_player {
                            let mut rotation_degree = (rotation * 180.0 / std::f32::consts::PI).round() % 360.0;
                            let mut player_degree = ((self.player.dir.y / self.player.dir.x).atan() * 180.0 / std::f32::consts::PI).round();

                            if rotation_degree < 0.0 {
                                rotation_degree = 360.0 + rotation_degree;
                            }

                            if (self.player.dir.x > 0.0 && self.player.dir.y < 0.0) || (self.player.dir.x > 0.0 && self.player.dir.y > 0.0) {
                                player_degree = 180.0 + player_degree;
                            } else if self.player.dir.x < 0.0 && self.player.dir.y > 0.0 {
                                player_degree = 360.0 + player_degree;
                            };

                            let mut value = -(4 + ((rotation_degree / 45.0).round() - ((player_degree - shift_degree) / 45.0).round()) as i32) % 8;
                            if value < 0 {
                                value = 8 + value;
                            }

                            textures::get_soldier_pixel(text_x + 64 * value as u32, text_y + 64 * self.player.frame)
                        } else {
                            textures::get_sprite_pixel(text_x, text_y + 64 * sprite.value)
                        };
                        if color.a != 0 {
                            self.canvas.put_pixel(x as usize, y as usize, color);
                        }
                    }
                }
            }
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.map.update(delta);
        self.player.update(&mut self.map, delta);

        self.draw_view();
        self.draw_sprites();
        self.canvas.update();
    }
}