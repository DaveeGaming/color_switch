use std::cmp::max;
use std::cmp::min;

use macroquad::prelude::*;
use macroquad::audio::*;

use crate::game::*;

impl Game {
    pub fn collection_update(&mut self) {
        self.background_update();
        let interact = is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter);
        let up = is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up);
        let down = is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down);
        let left = is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left);
        let right = is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right);

        if is_key_pressed(KeyCode::Q) {
            self.menu_selected = max(0, self.menu_selected - 1);
        }
        if is_key_pressed(KeyCode::E) {
            self.menu_selected = min(2, self.menu_selected + 1);
        }

        if is_key_pressed(KeyCode::Escape) {
            self.menu_selected = 3;
            self.game_state = GameState::MainMenu;
        }

        if up && self.collection_x - 5 >= 0 {
            self.collection_x -= 5;
        }

        if left {
            self.collection_x = max(self.collection_x - 1, 0)
        }

        match self.menu_selected {
            0 => { // Items
            },
            1 => { // Enemies
                if right {
                    self.collection_x = min(self.collection_x + 1, self.enemy_list.len() as i32 - 1)
                }

                if down && self.enemy_list.len() as i32 - 1 >= self.collection_x + 5 {
                    self.collection_x += 5;
                }
            }
            2 => { // Bosses

            }
            _ => { // what

            }
        }
    }

    pub fn collection_draw(&mut self) {
        clear_background(BLACK);
        self.background_draw();

        self.help_text();
        draw_text_ex("Q", 100.0, 150.0, TextParams { font: Some(&self.assets.font_monogram), font_size: 15, ..Default::default()});
        let text = vec![ "Items", "Enemies", "Bosses"];
        for i in 0..text.len() {
            let t = text[i];
            if self.menu_selected == i as i32 {
                let t = &format!("[{}]", t);
                draw_text_centered(t, 350.0 + (i as f32 * 250.0), 150.0, 16.0, &self.assets.font_monogram);
            } else {
                draw_text_centered(t, 350.0 + (i as f32 * 250.0), 150.0, 15.0, &self.assets.font_monogram);
            }
        }
        draw_text_ex("E", 1050.0, 150.0, TextParams { font: Some(&self.assets.font_monogram), font_size: 15, ..Default::default()});

        let start_pos = Vec2 { x: 100.0, y: 240.0};
        let offset = 160.0;
        match self.menu_selected {
            0 => { // Items

            },
            1 => { // Enemies
                for i in 0..self.enemy_list.len() {
                    let e = self.enemy_list[i];
                    let x = start_pos.x + ((i as f32 % 5.0) * offset);
                    let y = start_pos.y + ((i as f32 / 5.0).floor() * offset);
                    if i as i32 == self.collection_x {
                        draw_texture_ex(&self.assets.t.border, x - 10.0, y - 10.0, WHITE, 
                            DrawTextureParams { dest_size: Some( Vec2 { x: 120.0, y: 120.0}), ..Default::default() })
                    }
                    draw_rectangle(x, y, 100.0, 100.0, WHITE);
                }
            }
            2 => { // Bosses

            }
            _ => { // what

            }
        }
    }
}