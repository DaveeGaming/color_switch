use std::{ffi::FromVecWithNulError, sync::Arc};

use macroquad::prelude::*;
use rand::gen_range;

use crate::{colors::ColorState, game::*};

#[derive(Clone, Copy)]
pub enum ParticleShape {
    Circle,
    Triangle,
    Cube
}

#[derive(Clone, Copy)]
pub struct ParticleData {
    pub x: f32,
    pub y: f32,
    pub rot: f32,
    pub dx: f32,
    pub dy: f32,
    pub drot: f32,
    pub color: Color,
    pub speed: f32,
    pub shape: ParticleShape,

    pub lifetime_max: f32,
    pub lifetime_t: f32,

    pub opacity: f32,
    pub opacity_start: Option<f32>,
    pub opacity_end: Option<f32>,

    pub size: f32,
    pub size_start: Option<f32>,
    pub size_end: Option<f32>,
}

impl ParticleData {
    pub fn update(&mut self) {
        let time_ratio = 1.0 - (self.lifetime_t / self.lifetime_max);
        let dt = get_frame_time();

        self.lifetime_t -= dt;

        self.x += self.dx * self.speed * dt;
        self.y += self.dy * self.speed * dt;
        self.rot += self.drot * self.speed * dt;

        if self.size_start.is_some() {
            self.size = lerp(self.size_start.unwrap(), self.size_end.unwrap(), time_ratio);
        }

        if self.opacity_start.is_some() {
            self.opacity = lerp(self.opacity_start.unwrap(), self.opacity_end.unwrap(), time_ratio);
            self.color.a = self.opacity;
        }
    }

    pub fn draw(&self) {
        match self.shape {
            ParticleShape::Circle => {
                draw_circle(self.x, self.y, self.size/2.0, self.color);
            },
            ParticleShape::Cube => {
                draw_rectangle_ex(
                    self.x, 
                    self.y, 
                    self.size, self.size, 
                    DrawRectangleParams {
                        color: self.color,
                        rotation: self.rot,
                        offset: Vec2 { x: 0.5, y: 0.5 }
                    });
            },
            ParticleShape::Triangle => {
            }
        }
    }
}

#[derive(Clone)]
pub struct Particle {
    pub data: ParticleData,
    pub death_fun: Option<Arc<dyn FnMut(ParticleData, &mut Game)>>
}


impl Particle {
    pub fn default() -> Particle {
        Self {
            data: ParticleData {
                x: 0.0, y: 0.0, dx: 0.0, dy: 0.0,
                size: 10.0, lifetime_max: 1.0, lifetime_t: 1.0,
                rot: 0.0, drot: 0.0,
                speed: 50.0,
                opacity: 1.0,
                color: WHITE,
                shape: ParticleShape::Circle,
                opacity_start: None,
                opacity_end: None,
                size_start: None,
                size_end: None,
            },
            death_fun: None,
        }
    }

    pub fn new(x: f32, y: f32, dx: f32, dy: f32) -> Particle {
        let mut p = Self::default();
        p.data.x = x;
        p.data.y = y;
        p.data.dx = dx;
        p.data.dy = dy;
        p
    }
    pub fn with_pos(mut self, x: f32, y: f32) -> Self {
        self.data.x = x;
        self.data.y = y;
        self
    }

    pub fn with_fn(mut self, f: impl FnMut(ParticleData, &mut Game) + 'static) -> Self {
        self.death_fun = Some(Arc::new(f));
        self
    }
    pub fn with_shape(mut self, s: ParticleShape) -> Self {
        self.data.shape = s;
        self
    }

    pub fn with_color(mut self, c: Color) -> Self {
        self.data.color = c;
        self
    }

    pub fn with_data(mut self, d: ParticleData) -> Self {
        self.data = d;
        self.data.lifetime_t = d.lifetime_max;
        self
    }

    pub fn with_lifetime(mut self, t: f32) -> Self {
        self.data.lifetime_max = t;
        self.data.lifetime_t = t;
        self
    }

    pub fn with_opacity_dx(mut self, start: f32, end: f32) -> Self {
        self.data.opacity = start;
        self.data.opacity_start = Some(start);
        self.data.opacity_end = Some(end);
        self
    }

    pub fn with_speed(mut self, speed: f32) -> Self {
        self.data.speed = speed;
        self
    }

    pub fn with_dir(mut self, dx: f32, dy: f32) -> Self {
        self.data.dx = dx;
        self.data.dy = dy;
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.data.size = size;
        self
    }

    pub fn with_size_dx(mut self, start: f32, end: f32) -> Self {
        self.data.size = start;
        self.data.size_start = Some(start);
        self.data.size_end = Some(end);
        self
    }

    pub fn update(&mut self, g: &mut Game) {
        self.data.update();
        if self.death_fun.is_some() && self.data.lifetime_t < 0.0 {
            Arc::get_mut(&mut self.death_fun.as_mut().unwrap()).unwrap()(self.data, g);
        }

    }

    pub fn draw(&self) {
        self.data.draw();
    }
}

impl Game {
    pub fn particle_spawn(&mut self, x: f32, y: f32) {
        let big_particle = 
            Particle::default()
                .with_pos(x, y)
                .with_lifetime(0.2);
            

        let up = Vec2 { x: 0.0, y: -1.0 };
        for _ in 0..1 {
            let angle = gen_range(-3.0, 3.0);
            let speed = gen_range(50.0, 3000.0);
            let size = gen_range(40.0, 80.0);
            let opacity = gen_range(2.0, 0.5);
            let dir = rotate_vec(up, angle);
            self.particles.push(
                big_particle.clone()
                    .with_speed(speed)
                    .with_size_dx(size, 0.0)
                    .with_opacity_dx(opacity, 0.0)
                    .with_dir(dir.x, dir.y)
            );
        }
    }

    pub fn particle_death(&mut self, x: f32, y: f32) {
        self.particles.push(
            Particle::default()
                .with_size_dx(50.0, 0.0)
                .with_lifetime(0.2)
                .with_opacity_dx(2.0, 0.0)
                .with_pos(x,y)
        );

        let up = Vec2 { x: 0.0, y: 1.0 };
        let mini_particle = 
                Particle::default()
                    .with_pos(x,y)
                    .with_size_dx(8.0, 8.0)
                    .with_lifetime(0.4)
                    .with_opacity_dx(2.0, 0.0);

        for i in 0..10 {
            let rand = gen_range(0, 360);
            let dir = rotate_vec(up, rand as f32);
            let speed = gen_range(100.0, 200.0);
            self.particles.push(
                mini_particle.clone()
                    .with_dir(dir.x, dir.y)
                    .with_speed(speed)
            );
        }
    }


    pub fn particle_update(&mut self) {
        let color = match self.color_state {
            ColorState::Primary => self.palette.fg_primary,
            ColorState::Secondary => self.palette.fg_secondary
        };

        if is_key_down(KeyCode::G) {
            self.particle_spawn(self.mouse_pos.x, self.mouse_pos.y);
        }


        let mut particles = std::mem::take(&mut self.particles);
        for p in &mut particles {
            p.update(self);
        }
        self.particles.append(&mut particles);

        self.particles.retain_mut( |p| {
            p.data.lifetime_t > 0.0
        });
    }

    pub fn particle_draw(&mut self) {
        for p in &self.particles {
            p.draw();
        }
    }
}