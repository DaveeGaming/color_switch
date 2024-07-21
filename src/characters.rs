use std::cmp::max;
use std::cmp::min;

use macroquad::prelude::*;
use macroquad::audio::*;

use crate::assets::Assets;
use crate::game::*;
use crate::player::Player;

#[derive(PartialEq, Eq)]
pub enum CharacterKind {
    Garry,
    BobBobBob,
    John,
    Mark,
    Locked
}
pub struct Character {
    pub p: Player,
    pub name: String,
    pub health: i32,
    pub damage: i32,
    pub speed: f32,
    pub max_diff: i32,
    pub kind: CharacterKind
}

impl Character {
    pub fn get_sprite<'a>(&'a self, s: &'a Assets) -> &Texture2D {
        match self.kind {
            CharacterKind::Garry => &s.t.garry,
            CharacterKind::BobBobBob => &s.t.bobbobbob,
            CharacterKind::John => &s.t.john,
            CharacterKind::Mark => &s.t.mark,
            CharacterKind::Locked => &s.t.locked,
        }
    }
}


impl Game {
    pub fn characters_update(&mut self) {
        self.background_update();
        let interact = is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter);
        let up = is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up);
        let down = is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down);
        let left = is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left);
        let right = is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right);
        
        if self.menu_selected == 1 {
            if left {
                self.difficulty_select = max(0, self.difficulty_select - 1);
            }

            if right {
                self.difficulty_select = min(self.characters[self.selected_char as usize].max_diff + 2, self.difficulty_select + 1);
            }

            if interact {
                self.game_state = GameState::Playing;
                stop_sound(&self.assets.menu_song);
                play_sound(&self.assets.play_song, PlaySoundParams { looped: true, volume: self.music_level as f32 / 10.0});
            }
        }

        if self.menu_selected == 0 {
            if left {
                self.selected_char = max(0, self.selected_char - 1);
                self.difficulty_select = min(self.characters[self.selected_char as usize].max_diff + 2, self.difficulty_select);
                self.menu_switch();
            }
            if right {
                self.selected_char = min(self.characters.len() as i32 - 1, self.selected_char + 1);
                self.difficulty_select = min(self.characters[self.selected_char as usize].max_diff + 2, self.difficulty_select);
                self.menu_switch();
            }
            if interact {
                self.menu_selected += 1;
                self.menu_switch();
            }

            if is_key_pressed(KeyCode::Escape) {
                self.game_state = GameState::MainMenu;
                self.menu_selected = 0;
                self.menu_switch();
            }
        } else {
            if is_key_pressed(KeyCode::Escape) {
                self.menu_selected -= 1;
                self.menu_switch();
            }
        }

    }

    pub fn characters_draw(&mut self) {
        let x_center = DESIGN_WIDTH / 2.0;
        clear_background(BLACK);
        self.background_draw();
        self.help_text();

        let c = &self.characters[self.selected_char as usize];
        let texture = c.get_sprite(&self.assets);

        draw_text_centered(&c.name, x_center, 100.0, 30.0, &self.assets.font_monogram);
        draw_texture(texture, x_center - 100.0, 200.0, WHITE);
        let diff_text = match self.difficulty_select {
            0 => "Easy",
            1 => "Normal",
            2 => "Hard",
            3 => "Hard+",
            4 => "Hard++",
            5 => "Hard+++",
            _ => "lol"
        };
        if self.menu_selected == 1 {
            draw_text_centered(&format!("> {} <",diff_text), x_center, 500.0, 15.0, &self.assets.font_monogram);
        } else {
            draw_text_centered(diff_text, x_center, 500.0, 15.0, &self.assets.font_monogram);
        }


        draw_text_ex("Stats", 150.0, 240.0, TextParams { font: Some(&self.assets.font_monogram), font_size: 15, ..Default::default()});
        draw_text_ex("Health: 100", 150.0, 300.0, TextParams { font: Some(&self.assets.font_monogram), font_size: 10, ..Default::default()});
        draw_text_ex("Damage: 1", 150.0, 340.0, TextParams { font: Some(&self.assets.font_monogram), font_size: 10, ..Default::default()});
        draw_text_ex("Speed: 10", 150.0, 380.0, TextParams { font: Some(&self.assets.font_monogram), font_size: 10, ..Default::default()});

        for i in 0..self.characters.len() {
            let c = &self.characters[i];
            
            let texture = c.get_sprite(&self.assets);

            if self.menu_selected == 0 {
                if i == self.selected_char as usize {
                    let color = if c.kind == CharacterKind::Locked { Color::from_hex(0x4f4f4f)} else { WHITE };
                    draw_texture_ex(&self.assets.t.border, 200.0 + (i as f32 * 150.0) - 10.0, 600.0 - 10.0 , color, 
                        DrawTextureParams {
                            dest_size: Some(Vec2 { x: 100.0, y: 100.0 }),
                            ..Default::default()
                        }
                    );
                }
            }



            draw_texture_ex(texture, 200.0 + (i as f32 * 150.0), 600.0 , WHITE, 
                DrawTextureParams {
                    dest_size: Some(Vec2 { x: 80.0, y: 80.0 }),
                    ..Default::default()
                }
            )
        }
    }
}