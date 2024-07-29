use std::ffi::FromVecWithNulError;

use macroquad::prelude::*;

use crate::game::*;

pub enum ParticleShape {
    Circle,
    Triangle,
    Cube
}

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub color: Color,
    pub speed: f32,
    pub size: f32,
    pub lifetime_max: f32,
    pub lifetime_t: f32,
    pub shape: ParticleShape,
    pub opacity_start: Option<f32>,
    pub opacity_end: Option<f32>,
    pub size_start: Option<f32>,
    pub size_end: Option<f32>,
}


impl Particle {
    pub fn default() -> Particle {
        Self {
            x: 0.0, y: 0.0, dx: 0.0, dy: 0.0,
            size: 10.0, lifetime_max: 1.0, lifetime_t: 1.0,
            speed: 50.0,
            color: WHITE,
            shape: ParticleShape::Circle,
            opacity_start: None,
            opacity_end: None,
            size_start: None,
            size_end: None,
        }
    }

    pub fn new(x: f32, y: f32, dx: f32, dy: f32) -> Particle {
        let mut p = Self::default();
        p.x = x;
        p.y = y;
        p.dx = dx;
        p.dy = dy;
        p
    }

    pub fn with_shape(mut self, s: ParticleShape) -> Self {
        self.shape = s;
        self
    }

    pub fn with_opacity(mut self, start: f32, end: f32) -> Self {
        self.opacity_start = Some(start);
        self.opacity_end = Some(end);
        self
    }

    pub fn with_size(mut self, start: f32, end: f32) -> Self {
        self.size_start = Some(start);
        self.size_end = Some(end);
        self
    }

    pub fn update(&mut self) {
        let time_ratio = (self.lifetime_t / self.lifetime_max);
        let dt = get_frame_time();

        self.x += self.dx * self.speed * dt;
        self.y += self.dy * self.speed * dt;


    }

    pub fn draw(&self) {
        match self.shape {
            ParticleShape::Circle => {
                draw_circle(self.x, self.y, self.size/2.0, self.color);
            },
            ParticleShape::Cube => {
                draw_rectangle(self.x - self.size / 2.0, self.y - self.size / 2.0, self.size, self.size, self.color);
            },
            ParticleShape::Triangle => {
            }
        }
    }
}

impl Game {
    pub fn particle_update(&mut self) {
        self.particles.retain_mut( |p| {
            p.update();

            p.lifetime_t > 0.0
        });
    }

    pub fn particle_draw(&mut self) {
        for p in &self.particles {
            p.draw();
        }
    }
}