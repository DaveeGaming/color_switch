use macroquad::prelude::*;
use crate::{colors::ColorState, game::*};

impl Game {
    pub fn hp_bar_character(&self) {
        let color = self.get_primary_color();

        let mut bg_health = color;
        bg_health.a = 0.6;

        let mut hp = self.player.stats.current_health;
        let height = 70.0;
        let gap = -8.0;
        let width = height / 2.0;

        
        let offset = 20.0;
        let scale = 3.0;
        let texture = self.characters[self.selected_char as usize].get_sprite(&self.assets);
        
        let y = 50.0;
        for i in 0..self.player.stats.max_health {
            let x = offset + (self.player.size * scale) + 30.0 + (i as f32 * (width + gap));
            
            if hp > 0 {
                draw_texture_ex(&self.assets.t.hpbar, x, y, color, 
                    DrawTextureParams { dest_size: Some(Vec2 { x: width, y: height }), ..Default::default()});
                hp -= 1;
            } else {
                draw_texture_ex(&self.assets.t.hpbar, x, y, bg_health, 
                    DrawTextureParams { dest_size: Some(Vec2 { x: width, y: height }), ..Default::default()});
            }
        }

        draw_texture_ex(texture, offset, y, color, 
            DrawTextureParams { dest_size: Some(Vec2 { x: self.player.size * scale, y: self.player.size * scale }), ..Default::default()});

    }
}