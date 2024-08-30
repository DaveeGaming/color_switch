use macroquad::prelude::*;
use macroquad::audio::*;
use crate::game::*;
use crate::bullet::*;
use crate::colors::*;

pub struct PlayerStats {
    pub max_health: i32,
    pub current_health: i32,
    pub rerolls: i32,
    pub spread: f32,
    pub projectiles: i32,
    pub attack_speed: f32,
    pub movement_speed: i32,
    pub bullet_size: i32,
    pub bullet_damage: i32,
    pub bullet_speed: i32, // * 50
    pub heal_from_bullets: i32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            max_health: 10,
            current_health: 10,
            spread: 3.0,
            movement_speed: 5, // * 50
            projectiles: 1,
            bullet_damage: 1,
            bullet_size: 6,
            bullet_speed: 10, // * 50
            rerolls: 3,
            attack_speed: 1.0, // 1 / x
            heal_from_bullets: 1,
        }
    }
}

pub struct Player {
    pub stats: PlayerStats,
    pub rotation: f32,
    pub size: f32,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub shoot_dx: f32,
    pub shoot_dy: f32,
    pub shoot_t: f32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            stats: PlayerStats::default(),
            rotation: 0.0,
            size: 40.0,
            x: DESIGN_WIDTH/2.0,
            y: 700.0,
            dx: 0.0,
            dy: 0.0,
            shoot_dx: 1.0,
            shoot_dy: 0.0,
            shoot_t: 0.0,
        }
    }
}

impl Player {
    pub fn get_rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.size,
            h: self.size
        }
    }
}

impl Game {

    pub fn player_draw(&self) {
        let color = self.get_primary_color();

        let center_x = self.player.x + self.player.size/2.0;
        let center_y = self.player.y + self.player.size/2.0;

        let mut melee_color = color.clone();
        melee_color.a = 0.5;


        // if self.player.melee_t > 1.5 && self.player.melee_t < 2.0 {
        //     draw_circle(center_x, center_y, self.player.melee_range, melee_color);
        // }

        // draw_rectangle_ex(self.player.x, self.player.y, self.player.size, self.player.size,
        //     DrawRectangleParams {
        //         color: color,
        //         ..Default::default()
        //     }
        // );

            
        let offset = 20.0;
        let scale = 3.0;
        draw_circle(self.player.x + self.player.size / 2.0, self.player.y + self.player.size/2.0, self.player.size / 2.0, color);

       

        draw_line(
            self.player.x + self.player.size/2.0, // x center 
            self.player.y + self.player.size/2.0, // y center
            self.player.x + self.player.size/2.0 + self.player.shoot_dx * self.player.size/4.0, // x center + x_dir
            self.player.y + self.player.size/2.0 + self.player.shoot_dy * self.player.size/4.0, // y center + y_dir 
            2.0, WHITE);

        self.hp_bar_character();
    }


    pub fn player_update(&mut self) {
        let dt = get_frame_time();

        let mut dir = Vec2::ZERO;

        if is_key_down(KeyCode::A) {
            dir.x = -1.0;
        }
        if is_key_down(KeyCode::D) {
            dir.x = 1.0;
        }
        if is_key_down(KeyCode::W) {
            dir.y = -1.0;
        }
        if is_key_down(KeyCode::S) {
            dir.y = 1.0;
        }
        dir = dir.normalize_or_zero();

        let mut shoot_dir = Vec2::ZERO;
        let mut shooting = false;
        if is_key_down(KeyCode::Left) {
            shoot_dir.x = -1.0;
            shooting = true;
        }
        if is_key_down(KeyCode::Right) {
            shoot_dir.x = 1.0;
            shooting = true;
        }
        if is_key_down(KeyCode::Up) {
            shoot_dir.y = -1.0;
            shooting = true;
        }
        if is_key_down(KeyCode::Down) {
            shoot_dir.y = 1.0;
            shooting = true;
        }
        shoot_dir = shoot_dir.normalize_or_zero();

        self.player.shoot_dx = shoot_dir.x;
        self.player.shoot_dy = shoot_dir.y;

        self.player.dx = dir.x * (self.player.stats.movement_speed * 50) as f32;
        self.player.dy = dir.y * (self.player.stats.movement_speed * 50) as f32;

        if self.player.shoot_t > 0.0 {
            self.player.shoot_t -= dt;
        }

        if shooting {
            if self.player.shoot_t <= 0.0 {

                play_sound(&self.assets.shoot, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0 });
                // rotate by spread randomly
                let deg_projectile = 2.0;
                let offset = deg_projectile/2.0;

                for i in 0..self.player.stats.projectiles as i32 {
                    
                    let actual_spread = rand::gen_range(-self.player.stats.spread, self.player.stats.spread);
                    let  dir = rotate_vec(Vec2 { x: self.player.shoot_dx, y: self.player.shoot_dy}, actual_spread);
                    let mut dir = rotate_vec(dir, -offset + i as f32*deg_projectile);
                    dir = dir.normalize_or_zero();

                    self.bullets.push(Bullet::new(
                        1, // damage
                        self.player.x + 15.0, // x
                        self.player.y + 15.0, // y
                        dir.x, // dx
                        dir.y, // dy
                        self.player.stats.bullet_size as f32,
                        (self.player.stats.bullet_speed * 50) as f32,
                        BulletType::Player, // kind
                    ));
                }



                self.player.shoot_t += self.player.stats.attack_speed;
            }

        }

        // if self.player.melee_t < 0.0 && (is_key_down(KeyCode::K) || is_key_down(KeyCode::G)) && false {
        //     self.player.melee_t = 2.0;
        // }

        // if self.player.melee_t >= 0.0 {
        //     self.player.melee_t -= get_frame_time();
        // }

        self.player.x += self.player.dx * dt;
        self.player.y += self.player.dy * dt;
    }
}