use macroquad::prelude::*;
use crate::game::*;
use crate::wave::*;


impl Game {

    pub fn spawn_upgrades(&mut self) {
        let padding = 120.0;
        let upg_size = 100.0;
        
        let total_size = upg_size * self.upgrade_count + padding * (self.upgrade_count - 1.0);
        let start = total_size/2.0;
        
        
        for i in 0..self.upgrade_count as i32 {
            let center_x = DESIGN_WIDTH/2.0;
            let x = center_x - start + i as f32*(upg_size + padding);
            self.upgrades.push(
                Collectibe {
                    x: x,
                    y: DESIGN_HEIGHT/2.0 - upg_size/2.0,
                    size: upg_size,
                    kind: self.upg_list[rand::gen_range(0, self.upg_list.len())],
                    should_exist: true,
                }
            )
        }
        self.move_player();
        self.wave.upgrades_spawned = true;
    }

    pub fn update_start_cube(&mut self, c: &mut Collectibe) {
        if rect_collide(Rect{x: c.x, y: c.y, w: c.size, h: c.size}, self.player.get_rect()) {
            self.wave.state = WaveState::Spawning;
            c.should_exist = false;
            self.wave.start_spawned = false;
        }
    }

    pub fn draw_start_cube(&mut self, c: &Collectibe) {
        draw_texture(&self.assets.start_cube, c.x, c.y, WHITE);
    }

    pub fn update_upgrade(&mut self, c: &mut Collectibe) {
        let hit = rect_collide(self.player.get_rect(), c.get_rect());
        if !hit {
            return; // early return, dotn care didnt ask + l + ratio
        }

        self.wave.state = WaveState::Spawning;
        self.wave.upgrades_spawned = false;
        self.wave.upgrade_picked = true;

    
        match c.kind {
            CollectibeKind::Maxhp => {
                self.player.max_health += 1;
            }
            CollectibeKind::Projectile => {
                let new_dmg = self.player.damage - 1.0;
                self.player.projectiles += 1.0;
                self.player.damage = if new_dmg <= 0.5 { 0.5 } else { new_dmg };
                self.player.spread += 3.0;
            },
            CollectibeKind::Size =>{
                self.player.bullet_size += 0.5;
            }
            CollectibeKind::Speed => {
                self.player.move_speed += 30.0;
            }
            CollectibeKind::Slowdmg => {
                self.player.damage += 1.0;
                self.player.bullet_speed -= 30.0;
            },
            _ => ()
        }
    }

    pub fn draw_upgrades(&mut self, c: &Collectibe) {
        // definitely not start cube or any other, upg logic matches
        // match upg with its texture, for now colored cube
        match c.kind {
            CollectibeKind::Maxhp => {
                draw_texture(&self.assets.maxhp, c.x, c.y, WHITE);
                draw_text_ex("+ maxhp", c.x - 40.0, c.y + 130.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                )
            },
            CollectibeKind::Projectile => {
                draw_texture(&self.assets.projectile, c.x, c.y, WHITE);
                draw_text_ex("+ shot", c.x - 40.0, c.y + 130.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                );
                draw_text_ex("+ spread", c.x - 40.0, c.y + 160.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                );
                draw_text_ex("- dmg", c.x - 40.0, c.y + 190.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                );
            },
            CollectibeKind::Speed => {
                draw_texture(&self.assets.speed, c.x, c.y, WHITE);
                draw_text_ex("+ speed", c.x - 40.0, c.y + 130.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                )
            },
            CollectibeKind::Size => {
                draw_texture(&self.assets.size, c.x, c.y, WHITE);
                draw_text_ex("+ size", c.x - 40.0, c.y + 130.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                )
            },
            CollectibeKind::Slowdmg => {
                draw_texture(&self.assets.slowdmg, c.x, c.y, WHITE);
                draw_text_ex("+ dmg", c.x - 40.0, c.y + 130.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                );
                draw_text_ex("- shot speed", c.x - 40.0, c.y + 160.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                )
            },
            _ => ()
        }
    }
}