use macroquad::prelude::*;
use crate::game::*;
use crate::colors::*;


#[derive(PartialEq, Eq)]
pub enum BulletType {
    Player,
    Enemy
}

pub struct Bullet {
    pub damage: i32,
    pub x: f32,
    pub y: f32,
    pub last_x: f32,
    pub last_y: f32,
    pub dx: f32,
    pub dy: f32,
    pub size: f32,
    pub speed: f32,
    pub state: ColorState,
    pub kind: BulletType,
    pub hit: bool
}


impl Bullet {
    pub fn new(damage: i32, x: f32, y: f32, dx: f32, dy: f32, size: f32, speed: f32, kind: BulletType) -> Bullet {
        Bullet {
            x,y,dx,dy,kind, damage, size, speed,
            last_x: x,
            last_y: y,
            hit: false,
            state: ColorState::Primary,
        }
    }

    pub fn update(&mut self) {
        self.last_x = self.x;
        self.last_y = self.y;
        let dt = get_frame_time();

        self.x += self.dx * self.speed * dt;
        self.y += self.dy * self.speed * dt;
    }
}

pub struct CircleAttack {
    pub x: f32,
    pub y: f32,
    pub hit: bool,
    pub radius: f32,
    pub color: ColorState,
}

impl Game {
    pub fn bullet_draw(&mut self, b: &Bullet) {
        if b.kind == BulletType::Player {
            draw_circle(b.x, b.y, b.size, WHITE);
        } else {
            let color = match b.state {
                ColorState::Primary => self.palette.fg_primary,
                ColorState::Secondary => self.palette.fg_secondary,
            };
            draw_circle(b.x, b.y, b.size, color);
        }
    }

    pub fn bullet_collision(&mut self, b: &mut Bullet) {
        if b.x < 0.0 || b.y < 0.0 || b.x > DESIGN_WIDTH || b.y > DESIGN_HEIGHT {
            b.hit = true;
            return;
        }


        match b.kind {
            BulletType::Player => self.bullet_enemy_coll(b),
            BulletType::Enemy => self.bullet_player_coll(b)
        }
    }
}