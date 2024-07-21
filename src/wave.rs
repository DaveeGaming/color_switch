use macroquad::prelude::*;

#[derive(PartialEq, Eq)]
pub enum WaveState {
    Start,
    Spawning
}

pub struct Wave {
    pub current: i32,
    pub state: WaveState,
    pub enemy_remaining: i32,
    pub upgrade_picked: bool,
    pub enemies_set: bool,

    pub move_player:bool,
    pub move_player_t: f32,
    pub move_player_tmax: f32,
    pub old_x: f32,
    pub old_y: f32,

    pub spawn_delay_t: f32,
    pub spawn_delay_tmax: f32,

    pub start_spawned: bool,
    pub upgrades_spawned: bool,
}

impl Wave {
    pub fn default() -> Self {
        Wave { 
            current: 1, 
            state: WaveState::Start, 
            enemy_remaining: 0, 
            enemies_set: false,
            start_spawned: false, 
            spawn_delay_tmax: 5.0,
            spawn_delay_t: 0.0,
            upgrades_spawned: false, 
            upgrade_picked: false,

            move_player: false,
            move_player_t: 0.0,
            move_player_tmax: 0.8,
            old_x: 0.0, old_y: 0.0,
        }
    }
}