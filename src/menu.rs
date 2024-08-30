use std::cmp::max;
use std::cmp::min;

use macroquad::prelude::*;
use macroquad::audio::*;

use crate::game::*;

impl Game {

    pub fn background_update(&mut self) {
        if self.menu_bg_x > -100.0 && self.menu_bg_dx > 0.0 {
            self.menu_bg_dx = -30.0;
        }

        if self.menu_bg_x < -400.0 && self.menu_bg_dx < 0.0 {
            self.menu_bg_dx = 30.0;
        }

        if self.menu_bg_y > -100.0 && self.menu_bg_dy > 0.0 {
            self.menu_bg_dy = -30.0;
        }

        if self.menu_bg_y < -400.0 && self.menu_bg_dy < 0.0 {
            self.menu_bg_dy = 30.0;
        }

        self.menu_bg_x += self.menu_bg_dx * get_frame_time();
        self.menu_bg_y += self.menu_bg_dy * get_frame_time();
    }

    pub fn background_draw(&mut self) {
        draw_texture(&self.assets.t.bg1, self.menu_bg_x, self.menu_bg_y, self.palette.fg_primary);
    }


    pub fn menu_switch(&self) {
        play_sound(&self.assets.menu_switch, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0});
    }

    pub fn menu_update(&mut self) {
        self.background_update();

        let interact = is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter);
        let up = is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up);
        let down = is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down);
        let left = is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left);
        let right = is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right);



        if !self.menu_song_started {
            play_sound(&self.assets.menu_song, PlaySoundParams { looped: true, volume: self.music_level as f32 / 10.0});
            self.menu_song_started = true;
        }

        if up {
            self.menu_selected = max(0, self.menu_selected - 1);
            self.menu_switch();
        }

        if down {
            self.menu_selected = min(4, self.menu_selected + 1);
            self.menu_switch();
        }

        if self.menu_selected == 0 {
            if interact {
                self.game_state = GameState::Characters;
                // stop_sound(&self.assets.menu_song);
                // play_sound(&self.assets.play_song, PlaySoundParams { looped: true, volume: self.music_level as f32 / 10.0});
                self.menu_switch();
            }
        }

        if self.menu_selected == 1 {
            if interact {
                self.game_state = GameState::Options;
                self.menu_selected = 0;
                self.menu_switch();
            }
        }
        
        if self.menu_selected == 2 {
            if left { 
                self.curr_palette_idx -= 1;
                if self.curr_palette_idx < 0 { 
                    self.curr_palette_idx = self.palettes.len() as i32 - 1;
                }
                self.menu_switch();
            }

            if right { 
                self.curr_palette_idx += 1;
                if self.curr_palette_idx > self.palettes.len() as i32 - 1 { 
                    self.curr_palette_idx = 0;
                }
                self.menu_switch();
            }
            
            self.palette = match self.curr_palette_idx {
                1 => if true { self.palettes[1] } else { self.palettes[0] }
                2 => if true { self.palettes[2] } else { self.palettes[0] }
                _ => self.palettes[0],
            }
        }

        if self.menu_selected == 3 {
            if interact {
                self.game_state = GameState::Collection;
                self.menu_selected = 0;
                self.menu_switch();
            }
        }
    }

    pub fn help_text(&mut self) {
        draw_text_ex("WASD - navigate", 50.0, 800.0, 
            TextParams { font: Some(&self.assets.font_monogram), font_size: 7, color: GRAY, ..Default::default()});
        draw_text_ex("ESCAPE - return", 50.0, 780.0, 
            TextParams { font: Some(&self.assets.font_monogram), font_size: 7, color: GRAY, ..Default::default()});
        draw_text_ex("Space/Enter - confirm", 50.0, 760.0, 
            TextParams { font: Some(&self.assets.font_monogram), font_size: 7, color: GRAY, ..Default::default()});
    }

    pub fn menu_draw(&mut self) {
        let font_size = 15.0;

        clear_background(BLACK);
        let x_center = DESIGN_WIDTH/2.0;
        self.background_draw();
        self.particle_draw();
        draw_texture(&self.assets.t.menu1, x_center - 80.0, 100.0, self.palette.fg_primary);
        draw_texture(&self.assets.t.menu2, x_center - 80.0, 100.0, self.palette.fg_secondary);
        draw_text_centered(" COLOR  SWITCH ", x_center, 120.0, 30.0, &self.assets.font_monogram);
        draw_text_centered(&format!("Highscore: {} ", self.high_score), x_center, 220.0, 15.0, &self.assets.font_monogram);
        self.help_text();


        let menu_txt = vec![
            String::from("Play"),
            String::from("Options"),
            String::from("Color palette"),
            String::from("Collection"),
            String::from("Credits"),
        ];
        // let menu_txt = vec![
        //     String::from("Play"),
        //     format!("Music [{}]", Game::level_bar(self.music_level)),
        //     format!("Effects [{}]", Game::level_bar(self.effect_level)),
        // ];

        for i in 0..menu_txt.len() {
            let mut text: String;
            if self.menu_selected == 2 && i == 2 {
                text = match self.curr_palette_idx {
                    0 => "Red & Blue".to_string(),
                    1 => if true { "Orange & Green".to_string() } else { "Reach wave 10 to unlock".to_string() }
                    2 => if true { "Purple & Yellow".to_string() } else { "Reach wave 25 to unlock".to_string() }
                    _ => "what".to_string()
                };
                text = format!("> {} <", text);
            } else {
                text = if i == self.menu_selected as usize { format!("> {} <", menu_txt[i]) } else { menu_txt[i].to_string() };
            }
            draw_text_centered(&text, x_center, 540.0 + (i as f32 * 60.0), font_size, &self.assets.font_monogram);

        }
    }

        // PLAY
        // MUSIC
        // EFFECT
        // PALETTE

}   