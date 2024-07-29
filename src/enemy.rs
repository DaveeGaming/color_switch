use macroquad::prelude::*;
use crate::game::*;
use crate::bullet::*;
use crate::colors::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    FollowEnemy,
    FollowShootEnemy,
    StaticCircleAttack,
}

#[derive(Clone, Copy)]
pub struct Enemy {
    pub health: f32,
    pub x: f32,
    pub y: f32,
    pub rot: f32,
    pub size: f32,
    pub score: i32,
    pub state: ColorState,
    pub kind: EnemyType,
    pub attack_speed: f32,
    pub can_collide: bool,
    pub contact_damage: i32,
    pub attack_t: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: 10.0,
            x: 50.0,
            y: 50.0,
            rot: 0.0,
            size: 60.0,
            score: 10,
            can_collide: false,
            state: ColorState::Primary,
            kind: EnemyType::FollowEnemy,
            attack_speed: 0.0,
            attack_t: 0.0,
            contact_damage: 2,
        }
    }
}

impl Enemy {
    pub fn get_rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.size,
            h: self.size,
        }
    }
}

impl Game {
    pub fn update_follow_enemy(&mut self,e: &mut Enemy) {
        let dt = get_frame_time();
        let dir = dir_to_player(e.x, e.y, &self.player);
        let speed = 250.0;

        e.x += dir.x * speed * dt;
        e.y += dir.y * speed * dt;
    }

    pub fn draw_follow_enemy(&mut self,e: &mut Enemy) {
        let color = match e.state {
            ColorState::Primary => self.palette.fg_primary,
            ColorState::Secondary => self.palette.fg_secondary,
        };

        let e_pos = Vec2 { x: e.x, y: e.y};
        let p_pos = Vec2 { x: self.player.x, y: self.player.y};
        e.rot = (p_pos - e_pos).to_angle();

        draw_texture_sized(&self.assets.t.tower, e.x, e.y, color, e.size, e.rot);
    }

    pub fn update_follow_shoot_enemy(&mut self, e: &mut Enemy) {
        match self.color_state {
            ColorState::Primary => {
                // Chase player
                let dt = get_frame_time();
                let dir = dir_to_player(e.x, e.y, &self.player);
                let speed = 200.0;
                e.x += dir.x * speed * dt;
                e.y += dir.y * speed * dt;
            }
            ColorState::Secondary => {
                // Stop and shoot at player
                if e.attack_t <= 0.0 {
                    let dir = dir_to_player(e.x, e.y, &self.player);
                    self.bullets.push(
                        Bullet::new(1,e.x + e.size/2.0, e.y + e.size/2.0, dir.x, dir.y, 6.0, 550.0, BulletType::Enemy)
                    );
                    e.attack_t = e.attack_speed;
                }

            }
        } 

        let e_pos = Vec2 { x: e.x, y: e.y};
        let p_pos = Vec2 { x: self.player.x, y: self.player.y};
        e.rot = (p_pos - e_pos).to_angle();
    }

    pub fn draw_follow_shoot_enemy(&mut self,e: &mut Enemy) {
        draw_texture_sized(&self.assets.t.shooter, e.x, e.y, WHITE, e.size, e.rot);
        // draw_rectangle(e.x, e.y, e.size, e.size, WHITE); 
    }

    pub fn update_static_circle_enemy(&mut self,e: &mut Enemy) {
        let state = self.color_state.next();

        e.attack_t -= get_frame_time();
        if e.attack_t <= 0.0 {
            self.circle_attacks.push(
                CircleAttack { 
                    x: e.x + e.size/2.0, 
                    y: e.y + e.size/2.0, 
                    radius: 1.0, 
                    color: state,
                    hit: false,
                }
            );
            e.attack_t = e.attack_speed;
        }
    }

    pub fn draw_static_circle_enemy(&mut self,e: &mut Enemy) {
        let size = 10.0 + (e.attack_t / e.attack_speed * (e.size - 10.0));
        draw_texture_sized(&self.assets.t.tower, e.x - size/2.0 + e.size/2.0, e.y - size/2.0 + e.size/2.0, WHITE, size, e.rot);
        // draw_rectangle(e.x, e.y, e.size, e.size, YELLOW); 
    }
}