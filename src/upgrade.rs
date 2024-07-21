
use macroquad::prelude::*;
use crate::{assets::Assets, colors::ColorState, game::*, hot_lib, wave::WaveState};


pub struct Upgrade {
    pub name: String,
    pub description: String,
    pub lore: String,
    pub kind: UpgradeKind,
    pub rarity: UpgradeRarity,
}


#[derive(Clone, Copy)]
pub enum UpgradeKind {
    Speed,
    Projectile,
    SlowDamage
}

impl UpgradeKind {
    pub fn get_texture<'a>(&'a self, asset: &'a Assets) -> &Texture2D {
        match self {
            Self::Speed => &asset.t.speed,
            Self::Projectile => &asset.t.projectile,
            Self::SlowDamage => &asset.t.slowdmg,
        }
    }
}

#[derive(Clone, Copy)]
pub enum UpgradeRarity {
    Common = 10,
    Rare = 5,
    Epic = 3, 
    Legendary = 1,
}


pub struct UpgradeEntity {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub upg_index: usize
}

impl UpgradeEntity {
    pub fn get_rect(&self) -> Rect {
        Rect {
            x: self.x, y: self.y, w: self.size, h: self.size,
        }
    }
}

impl Game {
    pub fn update_upgrades(&mut self) {
        self.upgrade_shown = 1000;
        for upg in &self.upgrades {
            //let upg = &self.upg_list[upg.upg_index];
            let rect = upg.get_rect();

            if rect_collide(rect, self.player.get_rect()) {
                self.upgrade_shown = upg.upg_index;

                // Chose this upgrade, do the funi
                if is_key_pressed(KeyCode::Space) {

                }
            }
        }

        // draw_rectangle(50.0, 415.0, 300.0, 70.0, WHITE);  // Reroll
        let reroll_rect = hot_lib::get_reroll_rect();
        if rect_collide(self.player.get_rect(), reroll_rect) {
            self.upgrade_shown = 1001;
            if self.player.reroll > 0 && is_key_pressed(KeyCode::Space) {
                self.upgrades = Vec::new();
                self.spawn_upgrades();
                self.player.reroll -= 1;
            }
        }

        // draw_rectangle(50.0, 565.0, 450.0, 70.0, WHITE);  // Skip
        let skip_rect = hot_lib::get_skip_rect();
        if rect_collide(self.player.get_rect(), skip_rect) {
            self.upgrade_shown = 1002;
            if is_key_pressed(KeyCode::Space) {
                self.wave.state = WaveState::Spawning;
                self.wave.upgrades_spawned = false;
                self.wave.upgrade_picked = true;
            }
        }
    }

    pub fn draw_upgrades(&self) {
        let mut color = match self.color_state {
            ColorState::Primary => self.palette.fg_primary,
            ColorState::Secondary => self.palette.fg_secondary
        };
        color.a = 0.3;
        // draw_text_centered_c(&"space to select", DESIGN_WIDTH / 2.0, 250.0, 15.0, &self.assets.font_monogram, color);

        for upg_e in &self.upgrades {
            let upg = &self.upg_list[upg_e.upg_index];

            draw_texture_ex(
                upg.kind.get_texture(&self.assets),
                upg_e.x,
                upg_e.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some( Vec2 { x: upg_e.size, y: upg_e.size} ),
                    ..Default::default()
                }
            );

            draw_text_centered(
                &upg.name,
                upg_e.x + upg_e.size / 2.0, 
                upg_e.y + upg_e.size,
                6.0, &self.assets.font_monogram);
        }

        let reroll_color = if self.upgrade_shown == 1001 { WHITE } else { Color { a: 0.2, ..WHITE } };
        let reroll = format!("{} remaining", self.player.reroll);
        let reroll_rect = hot_lib::get_reroll_rect();
        draw_texture(&self.assets.t.reroll, reroll_rect.x, reroll_rect.y + 5.0, reroll_color);
        draw_text_ex("reroll", reroll_rect.x + 70.0, reroll_rect.y, TextParams { font_size: 10, color: reroll_color, font: Some(&self.assets.font_monogram), ..Default::default()});
        draw_text_ex(&reroll, reroll_rect.x + 70.0, reroll_rect.y + 35.0, TextParams { font_size: 10, color: reroll_color, font: Some(&self.assets.font_monogram), ..Default::default()});


        let skip_rect = hot_lib::get_skip_rect();
        let skip_color = if self.upgrade_shown == 1002 { WHITE } else { Color { a: 0.2, ..WHITE } };
        draw_texture(&self.assets.t.skip, skip_rect.x, skip_rect.y + 5.0, skip_color);
        draw_text_ex("skip for", skip_rect.x + 70.0, skip_rect.y, TextParams { font_size: 10, color: skip_color, font: Some(&self.assets.font_monogram), ..Default::default()});
        draw_text_ex("30% heal & +1 maxhp", skip_rect.x + 70.0, skip_rect.y + 35.0, TextParams { font_size: 10, color: skip_color, font: Some(&self.assets.font_monogram), ..Default::default()});

        if self.upgrade_shown < 1000 {
            let upg = &self.upg_list[self.upgrade_shown];
            let padding = 200.0;
            let x_padding = 25.0;
            let width = 400.0;
            let height = DESIGN_HEIGHT - padding * 2.0;
            let x_start = DESIGN_WIDTH - width - x_padding;

            let texture = &self.assets.t.upgrade_frame;

            draw_texture_ex(
                texture,
                x_start,
                padding,
                WHITE,
                DrawTextureParams {
                    dest_size: Some( Vec2 { x: width, y: height} ),
                    ..Default::default()
                }
            );

            // // Top right icon
            // let icon_size = 75.0;
            // draw_texture_ex(
            //     upg.kind.get_texture(&self.assets), 
            //     x_start,
            //     padding,
            //     WHITE,
            //     DrawTextureParams {
            //         dest_size: Some( Vec2 { x: icon_size, y: icon_size} ),
            //         ..Default::default()
            //     }
            // );
            

            draw_text_centered(&upg.name, x_start + width / 2.0, padding + 50.0, 10.0, &self.assets.font_monogram);
            draw_text_centered_c(&upg.lore, x_start + width / 2.0, padding + height - 70.0, 6.0, &self.assets.font_monogram, GRAY);
            // draw_text_ex(&upg.name, x_start + icon_size, padding + size.height / 2.0 + 10.0, 
            //     TextParams { 
            //         font: Some(&self.assets.font_monogram),
            //         font_size: 10,
            //         ..Default::default()      
            //     }
            // );

            let text: Vec<&str> = upg.description.split(';').collect();
            let size = 20.0 * text.len() as f32;
            let mut idx = 0.0;
            for string in text {
                draw_text_centered(&string, x_start + width / 2.0, padding + height/2.0  + (idx * 40.0) - size, 10.0, &self.assets.font_monogram);
                idx += 1.0;
            }
        }
    }

    pub fn spawn_upgrades(&mut self) {
        let padding = 120.0;
        let upg_size = 100.0;
        
        let total_size = upg_size * self.upgrade_count + padding * (self.upgrade_count - 1.0);
        let start = total_size/2.0;
        
        // Copy our upgrade list as indexes so we can remove any already chosen upgrades
        let mut all_upgrades_idx = Vec::new();
        let mut sum = 0;
        for i in 0..self.upg_list.len() {
            all_upgrades_idx.push(i);

            // Calculate the max random number we can roll
            sum += self.upg_list[i].rarity as i32;
        }

        // Store our chosen upgrades
        let mut picked_upgrades_idx = Vec::new();

        for _ in 0..self.upgrade_count as i32 {
            let random = macroquad::rand::gen_range(1, sum);

            let mut local_sum = self.upg_list[ all_upgrades_idx[0] ].rarity as i32;
            let mut idx = 1;
            while random > local_sum {
                local_sum += self.upg_list[ all_upgrades_idx[idx] ].rarity as i32;
                idx += 1;
            }
            idx = idx - 1;
            
            // We rolled an upgrade, remove it from the list, and reduce the sum
            picked_upgrades_idx.push( all_upgrades_idx[idx] );
            sum -= self.upg_list[ all_upgrades_idx[idx] ].rarity as i32;
            all_upgrades_idx.remove(idx);
        }
        
        for i in 0..self.upgrade_count as i32 {
            let center_x = DESIGN_WIDTH/2.0;
            let x = center_x - start + i as f32*(upg_size + padding);
            self.upgrades.push(
                UpgradeEntity {
                    x, y: DESIGN_HEIGHT/2.0-upg_size/2.0,
                    size: upg_size,
                    upg_index: picked_upgrades_idx[i as usize],
                }
            )
        }
        self.move_player();
        self.wave.upgrades_spawned = true;
    }
}