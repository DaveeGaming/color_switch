use std::cmp::{max, min};

use macroquad::prelude::*;
use macroquad::audio::*;

use crate::game::*;

impl Game {
    pub fn level_bar(v: i32) -> String {
        let mut o = "I".repeat(v as usize);
        let dot = ".".repeat(10-v as usize);
        o.push_str(&dot);
        return o;
    }

    pub fn settings_update(&mut self) {
        self.background_update();
        let interact = is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter);
        let up = is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up);
        let down = is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down);
        let left = is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left);
        let right = is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right);

        if up {
            self.menu_selected = max(0, self.menu_selected - 1);
            self.menu_switch();
        }

        if down {
            self.menu_selected = min(3, self.menu_selected + 1);
            self.menu_switch();
        }

        if is_key_pressed(KeyCode::Escape) {
            self.game_state = GameState::MainMenu;
            self.menu_switch();
        }

        if self.menu_selected == 0 {
            if interact {
                self.game_state = GameState::MainMenu;
                self.menu_selected = 1;
                self.menu_switch();
            }
        }

        if self.menu_selected == 1 {
            if left {
                self.music_level = max(0, self.music_level - 1);
                set_sound_volume(&self.assets.menu_song, self.music_level as f32 / 10.0);
                self.menu_switch();
                self.should_save = true;
            }

            if right {
                self.music_level = min(10, self.music_level + 1);
                set_sound_volume(&self.assets.menu_song, self.music_level as f32 / 10.0);
                self.menu_switch();
                self.should_save = true;
            }
        }

        if self.menu_selected == 2 {
            if left { 
                self.effect_level = max(0, self.effect_level - 1);
                self.menu_switch();
                self.should_save = true;
            }

            if right { 
                self.effect_level = min(10, self.effect_level + 1);
                self.menu_switch();
                self.should_save = true;
            }
        }

        if self.menu_selected == 3 {
            if interact {
                self.shooting_sound = !self.shooting_sound;
                self.menu_switch();
                self.should_save = true;
            }
        }
    }

    pub fn settings_draw(&mut self, bg_color: Color) {
        clear_background(bg_color);
        self.background_draw();
        self.help_text();

        let font_size = 15.0;
        let x_center = DESIGN_WIDTH/2.0;

        let shoot_sound = if self.shooting_sound { String::from("Shooting sound  on") } else { String::from("Shooting sound  off") };
        let menu_txt = vec![
            String::from("Back"),
            format!("Music [{}]", Game::level_bar(self.music_level)),
            format!("Effects [{}]", Game::level_bar(self.effect_level)),
            shoot_sound, 
        ];

        for i in 0..menu_txt.len() {
            let text = if i == self.menu_selected as usize { format!("> {} <", menu_txt[i]) } else { menu_txt[i].to_string() };

            if !self.shooting_sound && i == 3 {
                draw_text_centered(&text, x_center + 12.5, 200.0 + (i as f32 * 60.0), font_size, &self.assets.font_monogram);
            } else {
                draw_text_centered(&text, x_center, 200.0 + (i as f32 * 60.0), font_size, &self.assets.font_monogram);
            }
        }
    }
}