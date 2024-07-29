use std::{cmp::min, sync::MutexGuard};

use macroquad::prelude::*;
use macroquad::audio::*;
use quad_storage::LocalStorage;
use rand::gen_range;

use crate::characters::Character;
use crate::characters::CharacterKind;
use crate::particle::Particle;
use crate::upgrade::*;
use crate::{assets::Assets, bullet::*, colors::ColorPalette, enemy::*};
use crate::player::*;
use crate::wave::*;
use crate::colors::*;

pub const DESIGN_WIDTH: f32 = 1600.;
pub const DESIGN_HEIGHT: f32 = 900.;

//TODO: Finish upgrade system
//TODO: SCore screen
//TODO: Web save data for sound level and high score
//TODO: make more upgrades,
//TODO: make a somewhat infinite scaling game
//TODO: make sprites for upgrades (draw or idk some shit)
//TODO: find a menu song and game song, maybe two, so it doesnt get so boring
//TODO: have sound levels in pause menu, quit button, and stats
//TODO: particle system for enemy death, and some enemy attacks
//TODO: have sound effects for dying, shooting,
//TODO: small upgrade hints
//TODO: lazer attack maybe?
//TODO: circle attack with radius check
//TODO: PLAYTEST WITH SOMEONE
//TODO: a bit interactive/visual main menu
//TODO: also a fucking game title lmao




pub enum GameState {
    MainMenu,
    Playing,
    Options,
    Collection,
    Characters,
    Score,
}


pub struct Unlocks {
    pub orangegreen: bool,
    pub purpleyellow: bool,
}


pub struct DebugStuff{
    pub debug1: i32,
    pub debug2: i32,
    pub debug3: i32,
}

impl DebugStuff {
    pub fn default() -> Self {
        DebugStuff {
            debug1: 0,
            debug2: 0,
            debug3: 0,
        }
    }
}


pub struct Game {
    pub game_state: GameState,
    pub color_state: ColorState,
    pub unlocks: Unlocks,
    pub high_score: i32,
    pub current_score: i32,
    pub assets: Assets,
    pub should_save: bool,
    pub palettes: [ColorPalette; 3],
    pub enemy_list:  [Enemy; 10],
    pub upg_list: [Upgrade; 4],
    pub palette: ColorPalette,
    pub curr_palette_idx: i32,
    pub enemy_spawn: Vec<SpawnEnemy>,
    pub enemies: Vec<Enemy>, // Box is for allocating to the heap
    pub characters: Vec<Character>,
    pub bullets: Vec<Bullet>,
    pub particles: Vec<Particle>,
    pub circle_attacks: Vec<CircleAttack>,
    pub upgrades: Vec<UpgradeEntity>, 
    pub player: Player,
    pub wave: Wave,
    pub upgrade_count: f32,
    pub selected_char: i32,
    pub debug: DebugStuff,
    pub upgrade_shown: usize,

    pub menu_bg_x: f32,
    pub menu_bg_y: f32,
    pub menu_bg_dx: f32,
    pub menu_bg_dy: f32,
    pub difficulty_select: i32,
    pub menu_selected: i32,
    pub music_level: i32,
    pub effect_level: i32,
    pub collection_x: i32,
    pub shooting_sound: bool,
    pub menu_song_started: bool,
    pub switch_effect_t: f32,
    pub switch_effect_total: f32,
}

impl Game {
    pub async fn default() -> Self {
        let mut g = Game {
            color_state: ColorState::Primary,
            assets: Assets::default().await,
            should_save: false,
            palette: ColorPalette::default(),
            curr_palette_idx: 0,
            debug: DebugStuff::default(),
            player: Player::default(),
            enemies: Vec::new(),
            particles: Vec::new(),
            enemy_spawn: Vec::new(),
            collection_x: 0,
            shooting_sound: true,
            menu_bg_dx: 30.0,
            menu_bg_dy: 30.0,
            menu_bg_x: -200.0,
            difficulty_select: 1,
            menu_bg_y: -300.0,
            bullets: vec![],
            circle_attacks: Vec::new(),
            characters: Vec::new(),
            upgrade_shown: 1000,
            upgrades: Vec::new(),
            menu_song_started: false,
            high_score: 0,
            current_score: 0,
            game_state: GameState::MainMenu,
            unlocks: Unlocks { 
                orangegreen: false,
                purpleyellow: false,
            },

            menu_selected: 0,
            music_level: 3,
            selected_char: 0,
            effect_level: 3,

            upgrade_count: 3.0,

            switch_effect_t: 0.0,
            switch_effect_total: 0.01,

            wave: Wave::default(),

            upg_list: [
                Upgrade {
                    name: String::from("Talaria"),
                    description: String::from("+10 speed"),
                    lore: String::from("Please don't wear socks with it"),
                    kind: UpgradeKind::Speed,
                    rarity: UpgradeRarity::Common,
                },
                Upgrade {
                    name: String::from("Hermes pants"),
                    description: String::from("+5 speed;-2 max hp"),
                    lore: String::from("hehe funi stuff hihi"),
                    kind: UpgradeKind::Speed,
                    rarity: UpgradeRarity::Common,
                },
                Upgrade {
                    name: String::from("Hermes cock"),
                    description: String::from("Every 2nd;attack deals;+2 damage"),
                    lore: String::from("hehe funi stuff hihi"),
                    kind: UpgradeKind::Speed,
                    rarity: UpgradeRarity::Common,
                },
                Upgrade {
                    name: String::from("Hermes cock II."),
                    description: String::from("desc"),
                    lore: String::from("hehe funi stuff hihi"),
                    kind: UpgradeKind::Speed,
                    rarity: UpgradeRarity::Common,
                },
                // CollectibeKind::Maxhp,
                // CollectibeKind::Projectile,
                // CollectibeKind::Size,
                // CollectibeKind::Slowdmg,
                // CollectibeKind::Speed,
            ],

            palettes: [
                ColorPalette::default(),
                ColorPalette::create_from(ORANGE, GREEN),
                ColorPalette::create_from(PURPLE, YELLOW)
            ],

            enemy_list: [
                Enemy { health: 5.0, x: 50.0, y: 50.0, size: 40.0, score: 15 , kind: EnemyType::FollowShootEnemy, attack_speed: 1.0, can_collide: true, ..Default::default()},
                Enemy { health: 10.0, x: 50.0, y: 50.0, size: 40.0, score: 30, kind: EnemyType::StaticCircleAttack, can_collide: true, contact_damage: 3, attack_t: 5.0, attack_speed: 5.0, ..Default::default()},
                Enemy { state: ColorState::Primary,health: 2.0, x: 50.0, y: 50.0, size: 20.0, score: 5, kind: EnemyType::FollowEnemy, can_collide: true, ..Default::default()},
                Enemy { state: ColorState::Secondary,health: 2.0, x: 50.0, y: 50.0, size: 20.0, score: 5, kind: EnemyType::FollowEnemy, can_collide: true, ..Default::default()},
                Enemy { state: ColorState::Secondary,health: 2.0, x: 50.0, y: 50.0, size: 20.0, score: 5, kind: EnemyType::FollowEnemy, can_collide: true, ..Default::default()},
                Enemy { state: ColorState::Secondary,health: 2.0, x: 50.0, y: 50.0, size: 20.0, score: 5, kind: EnemyType::FollowEnemy, can_collide: true, ..Default::default()},
                Enemy { state: ColorState::Secondary,health: 2.0, x: 50.0, y: 50.0, size: 20.0, score: 5, kind: EnemyType::FollowEnemy, can_collide: true, ..Default::default()},
                Enemy { state: ColorState::Secondary,health: 2.0, x: 50.0, y: 50.0, size: 20.0, score: 5, kind: EnemyType::FollowEnemy, can_collide: true, ..Default::default()},
                Enemy { state: ColorState::Secondary,health: 2.0, x: 50.0, y: 50.0, size: 20.0, score: 5, kind: EnemyType::FollowEnemy, can_collide: true, ..Default::default()},
                Enemy { state: ColorState::Secondary,health: 2.0, x: 50.0, y: 50.0, size: 20.0, score: 5, kind: EnemyType::FollowEnemy, can_collide: true, ..Default::default()},
            ]
        };
        g.characters = vec![
            Character {
               p: Player::default(),
               name: String::from("Garry"),
               health: 10,
               speed: 10.0,
               max_diff: 0,
               damage: 10,
               kind: CharacterKind::Garry,
            },
            Character {
               p: Player::default(),
               name: String::from("Bob, Bob & Bob"),
               health: 10,
               speed: 10.0,
               max_diff: 3,
               damage: 10,
               kind: CharacterKind::BobBobBob,
            },
            Character {
               p: Player::default(),
               name: String::from("John"),
               health: 10,
               max_diff: 1,
               speed: 10.0,
               damage: 10,
               kind: CharacterKind::John,
            },
            Character {
               p: Player::default(),
               name: String::from("Mark"),
               health: 10,
               max_diff: 0,
               speed: 10.0,
               damage: 10,
               kind: CharacterKind::Mark,
            },
            Character {
               p: Player::default(),
               name: String::from("Locked"),
               health: 10,
               max_diff: 1,
               speed: 10.0,
               damage: 10,
               kind: CharacterKind::Locked,
            },
            Character {
               p: Player::default(),
               name: String::from("Locked"),
               health: 10,
               max_diff: 1,
               speed: 10.0,
               damage: 10,
               kind: CharacterKind::Locked,
            },
        ];

        return g;    
    }
}


pub struct SpawnEnemy {
    pub x: f32,
    pub y: f32,
    pub spawn_t: f32,
}

impl Game {
    pub fn update(&mut self) {
        self.particle_update();
        match self.game_state {
            GameState::MainMenu => self.menu_update(),
            GameState::Playing => self.game_update(),
            GameState::Options => self.settings_update(),
            GameState::Characters => self.characters_update(),
            GameState::Collection => self.collection_update(),
            _ => ()
        }
    }

    pub fn draw(&mut self) {
        self.particle_draw();
        match self.game_state {
            GameState::MainMenu => self.menu_draw(),
            GameState::Playing => self.game_draw(),
            GameState::Options => self.settings_draw(BLACK),
            GameState::Characters => self.characters_draw(),
            GameState::Collection => self.collection_draw(),
            _ => ()
        }
    }




    // =========== MENU STATE ==============
    pub fn save_data(&mut self, s: &mut MutexGuard<LocalStorage>) {

        if self.should_save {
            s.set("highscore", &self.high_score.to_string());
            s.set("orangeyellow", &self.unlocks.orangegreen.to_string());
            s.set("purpleyellow", &self.unlocks.purpleyellow.to_string());
            s.set("sound_volume", &self.music_level.to_string());
            s.set("effect_volume", &self.effect_level.to_string());
        }
    }

    pub fn change_state(&mut self, s: GameState) {
        self.game_state = s;
    }


    // =========== ENEMY SPAWN ============

    pub fn update_spawning(&mut self, s: &mut SpawnEnemy) {
        s.spawn_t -= get_frame_time();

        if s.spawn_t <= 0.0 {
            //TODO: Randomly spawn here

            let to_spawn = min(5, self.wave.enemy_remaining);

            for _ in 0..to_spawn {
                let x = rand::gen_range(0.0, 300.0);
                let y = rand::gen_range(0.0, 300.0);
                let kind = rand::gen_range(0, self.enemy_list.len() - 1);

                
                let mut enemy = self.enemy_list[kind].clone();
                enemy.x = x + s.x;
                enemy.y = y + s.y;
                self.enemies.push( enemy );

                self.wave.enemy_remaining -= 1;
            }
        }
    }

    pub fn draw_spawning(&mut self,s: &mut SpawnEnemy) {
        let mut color = match self.color_state {
            ColorState::Primary => self.palette.fg_primary,
            ColorState::Secondary => self.palette.fg_secondary
        };

        let spawn_time = 2.0;
        let third = spawn_time / 3.0;
        color.a = 0.5;

        if s.spawn_t > 1.5 {
            draw_texture(&self.assets.t.enemy_spawn1, s.x, s.y, color);
        } else if s.spawn_t > 1.0 {
            draw_texture(&self.assets.t.enemy_spawn2, s.x, s.y, color);
        } else if s.spawn_t > 0.5 {
            draw_texture(&self.assets.t.enemy_spawn3, s.x, s.y, color);
        } else {
            draw_texture(&self.assets.t.enemy_spawn4, s.x, s.y, color);
        }

    }

    // =========== GAME STATE ==============

    // pub fn clean_map_and_move_player(&mut self) {
    //     self.bullets = Vec::new();
    //     self.player.x = DESIGN_WIDTH/2.0 - self.player.size/2.0;
    //     self.player.y = 600.0;
    // }

    pub fn move_player(&mut self) {
        self.bullets = Vec::new();
        self.circle_attacks = Vec::new();
        self.wave.move_player = true;
        self.wave.move_player_t = self.wave.move_player_tmax;
        self.wave.old_x = self.player.x;
        self.wave.old_y = self.player.y;
    }

    pub fn game_update(&mut self) {

        if is_key_pressed(KeyCode::Key1) { self.debug.debug1 = increment_or_zero(self.debug.debug1, 1); }
        if is_key_pressed(KeyCode::Key2) { self.debug.debug2 = increment_or_zero(self.debug.debug2, 4); }
        if is_key_pressed(KeyCode::Key3) { self.debug.debug3 = increment_or_zero(self.debug.debug3, 1); }

        if self.wave.current >= 10 {
            self.unlocks.orangegreen = true;
            self.should_save = true;
        }

        if self.wave.current >= 25 {
            self.unlocks.purpleyellow = true;
            self.should_save = true;
        }

        if self.player.health <= 0 || is_key_pressed(KeyCode::Escape) {
            if self.current_score > self.high_score {
                self.high_score = self.current_score;
                self.should_save = true;
            }

            self.player = Player::default();
            self.wave = Wave::default();
            self.enemies = Vec::new();
            self.bullets = Vec::new();
            self.circle_attacks = Vec::new();
            self.upgrades = Vec::new();
            self.enemy_spawn = Vec::new();
            self.current_score = 0;
            stop_sound(&self.assets.play_song);
            play_sound(&self.assets.menu_song, PlaySoundParams { looped: true, volume: self.music_level as f32 / 10.0 });
            self.game_state = GameState::MainMenu;
        }
        
        if self.wave.move_player {
            self.wave.move_player_t -= get_frame_time();
            
            let dest_x = DESIGN_WIDTH/2.0 - self.player.size/2.0;
            let dest_y = 700.0;
            
            let diff_x = dest_x - self.wave.old_x;
            let diff_y = dest_y - self.wave.old_y;
            
            if self.wave.move_player_t < 0.0 {
                self.wave.move_player = false;
                return;
            }
            
            let lerp_val = (self.wave.move_player_tmax - self.wave.move_player_t) / self.wave.move_player_tmax;
            self.player.x = self.wave.old_x + diff_x * lerp_val;
            self.player.y = self.wave.old_y + diff_y * lerp_val;


            return;
        }

        self.player_update();
        
        let mut spawners = std::mem::take(&mut self.enemy_spawn);
        spawners.retain_mut(|s| {
            self.update_spawning(s);
            
            s.spawn_t > 0.0
        }); 
        self.enemy_spawn = spawners;
        
        let mut enemies = std::mem::take(&mut self.enemies);
        enemies.retain_mut(|e| {
            if e.attack_t >= 0.0 {
                e.attack_t -= get_frame_time();
            }
            
            match e.kind {
                EnemyType::FollowEnemy => self.update_follow_enemy(e),
                EnemyType::FollowShootEnemy => self.update_follow_shoot_enemy(e),
                EnemyType::StaticCircleAttack => self.update_static_circle_enemy(e)
            }
            
            self.enemy_collision(e);
            
            if e.health <= 0.0 {
                self.current_score += e.score;   
                play_sound(&self.assets.dead, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0 })
            }
            e.health > 0.0
        });
        self.enemies = enemies;
        

        let mut circles = std::mem::take(&mut self.circle_attacks);
        circles.retain_mut(|c| {
            c.radius += get_frame_time() * 500.0 * (c.radius/200.0);

            if !c.hit {
                // check player distance
                let diffx = (c.x - self.player.x + self.player.size/2.0).abs();
                let diffy = (c.y - self.player.y + self.player.size/2.0).abs();
    
                // player/circle distance
                let dist = (diffx * diffx + diffy * diffy).sqrt() - c.radius;
                
                // player has size, and circle has thickness
                let circle_pad = 2.5;
                let player_pad = self.player.size/2.0;
                
                let dist = dist.abs();
                if dist < circle_pad + player_pad || dist <= circle_pad - player_pad || dist <= player_pad - circle_pad || dist == circle_pad + player_pad {
                    // inside player
                    if self.color_state != c.color {
                        c.hit = true;
                        play_sound(&self.assets.hit, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0 });
                        self.player.health -= 2;
                    } 
                }
            }

            c.radius < 2000.0
        });
        self.circle_attacks = circles;

        self.update_upgrades();
        self.upgrades.retain(|_| {
            !self.wave.upgrade_picked
        });


        let mut bullets = std::mem::take(&mut self.bullets);
        bullets.retain_mut(|b| {
            b.update();
            self.bullet_collision(b);

            !b.hit
        });
        self.bullets = bullets;

        if is_key_pressed(KeyCode::C) {
            self.palette = self.palettes[ rand::gen_range(0, self.palettes.len()) ]
        }

        if !(self.wave.state == WaveState::Start && self.wave.current > 0) && is_key_pressed(KeyCode::Space){
            if self.wave.current == 0 {
                self.switch_effect_total = 0.3;
            }
            // cool circle effect
            self.switch_effect_t = self.switch_effect_total;
        }
        
        if is_key_pressed(KeyCode::B) {
            self.enemies.push( 
                self.enemy_list[1]
            );
        }

        
        if self.switch_effect_t >= 0.0 {
            self.switch_effect_t -= get_frame_time();
        }
        if self.switch_effect_t <= 0.0 && self.switch_effect_t > -1.0 {
            self.color_state = self.color_state.next();
            self.switch_effect_t = -2.0;
            self.switch_effect_total = 0.0;
        }

        match self.wave.state {
            WaveState::Start => {
                if self.wave.current != 0 && !self.wave.start_spawned {
                    if !self.wave.upgrades_spawned {
                        self.spawn_upgrades();
                        self.move_player();
                    }
                }


                if self.wave.current == 0 && is_key_pressed(KeyCode::Space) {
                    self.wave.current = 1;
                    self.wave.state = WaveState::Spawning;
                }
            },
            WaveState::Spawning => {
                if self.switch_effect_t > 0.0 {
                    return;
                }

                // Wave started, everyting got defeated
                if !self.wave.enemies_set {
                    let enemies_to_spawn = 20 + self.wave.current * 3;
                    self.wave.enemy_remaining = enemies_to_spawn;
                    self.wave.spawn_delay_tmax = 5.0 - self.wave.current as f32 * 0.05;
                    self.wave.enemies_set = true;
                }

                self.wave.spawn_delay_t -= get_frame_time();
                if self.wave.spawn_delay_t > 0.0 && self.enemies.len() == 0 && self.enemy_spawn.len() == 0 {
                    self.wave.spawn_delay_t = 0.0;
                }

                if self.wave.spawn_delay_t <= 0.0 {
                    if self.wave.enemies_set && self.wave.enemy_remaining > 0 {
                        let rad_x = rand::gen_range(50.0, DESIGN_WIDTH - 400.0);
                        let rad_y = rand::gen_range(50.0,  DESIGN_HEIGHT - 350.0);
                        
                        self.enemy_spawn.push(
                            SpawnEnemy {
                                x: rad_x, y: rad_y,
                                spawn_t: 2.0,
                            }
                        );
                        
                        self.wave.spawn_delay_t = self.wave.spawn_delay_tmax;
                    }
                }



                if self.wave.enemy_remaining == 0 && self.enemy_spawn.len() == 0 && self.enemies.len() == 0 {
                    self.wave.state = WaveState::Start;
                    self.wave.current += 1;
                    self.wave.upgrade_picked = false;
                    self.wave.enemies_set = false;
                    self.wave.spawn_delay_t = 0.0;
                }
                
            }
        }
    }


    pub fn game_draw(&mut self) {


        let color = match self.color_state {
            ColorState::Primary => self.palette.fg_primary,
            ColorState::Secondary => self.palette.fg_secondary
        };

        let bg_color = match self.color_state {
            ColorState::Primary =>  self.palette.bg_primary,
            ColorState::Secondary => self.palette.bg_secondary
        };

        let bg_color_invert = match self.color_state {
            ColorState::Secondary =>  self.palette.bg_primary,
            ColorState::Primary => self.palette.bg_secondary
        };


        clear_background(bg_color);

        if self.wave.current == 0 {
            draw_texture(&self.assets.t.controls_finish, 0.0, 0.0, color);
        }

        // draw switch effect before everything else
        if self.switch_effect_t > 0.0 {
            draw_circle(self.player.x, self.player.y, 
                2000.0 * (self.switch_effect_total - self.switch_effect_t) / self.switch_effect_total,
                bg_color_invert);
        }

        let mut enemies = std::mem::take(&mut self.enemies);
        for e in enemies.iter_mut() {
            match e.kind {
                EnemyType::FollowEnemy => self.draw_follow_enemy(e),
                EnemyType::FollowShootEnemy => self.draw_follow_shoot_enemy(e),
                EnemyType::StaticCircleAttack => self.draw_static_circle_enemy(e)
            }
        }
        self.enemies = enemies;
        
        let mut circles = std::mem::take(&mut self.circle_attacks);
        for c in circles.iter_mut() {
            let color = match c.color {
                ColorState::Primary => self.palette.fg_primary,
                ColorState::Secondary => self.palette.fg_secondary
            };
            draw_circle_lines(c.x, c.y, c.radius, 5.0, color)
        }
        self.circle_attacks = circles;
        
        let mut bullets = std::mem::take(&mut self.bullets);
        for b in bullets.iter_mut() {
            self.bullet_draw(&b);
        }
        self.bullets = bullets;

        let mut spawners = std::mem::take(&mut self.enemy_spawn);
        for s in spawners.iter_mut() {
            self.draw_spawning(s);
        }
        self.enemy_spawn = spawners;

        self.draw_upgrades();
        self.player_draw();
        
        let x_center = DESIGN_WIDTH / 2.0;
        let wave_txt = format!("Wave {}", self.wave.current);
        draw_text_centered(&wave_txt, x_center, 50.0, 20.0, &self.assets.font_monogram);
        let score = format!("score: {}", self.current_score);
        draw_text_centered(&score, x_center, 110.0, 8.0, &self.assets.font_monogram);
    }
}



// =========== UTILS ============

pub fn dir_to_player(x: f32, y: f32, p: &Player) -> Vec2 {
    let diff = Vec2 { 
        x: p.x - x,
        y: p.y - y,
    };

    return diff.normalize_or_zero();
}

pub fn distance_to_player(x: f32, y: f32, p: &Player) -> f32 {
    let diff = Vec2 { 
        x: p.x - x,
        y: p.y - y,
    };

    return (diff.x * diff.x + diff.y * diff.y).sqrt();
}


pub fn draw_text_centered_c(text: &str, x: f32, y: f32, font_size: f32, font: &Font, color: Color) {
    let size = measure_text(&text, Some(&font), font_size as u16, 1.0);
    draw_text_ex(&text, x -size.width/2.0, y, TextParams { 
        font: Some(font), 
        color: color,
        font_size: font_size as u16,..Default::default() });
}

pub fn draw_text_centered(text: &str, x: f32, y: f32, font_size: f32, font: &Font) {
    let size = measure_text(&text, Some(&font), font_size as u16, 1.0);
    draw_text_ex(&text, x -size.width/2.0, y, TextParams { 
        font: Some(font), 
        font_size: font_size as u16,..Default::default() });
}

// Takes degrees
pub fn rotate_vec(v: Vec2, d: f32) -> Vec2 {
    let rad = d.to_radians();
    let mut dir = Vec2::ZERO;
    dir.x = v.x * rad.cos() - v.y * rad.sin();
    dir.y = v.x * rad.sin() + v.y * rad.cos();
    return dir
}

pub fn rect_collide(r1: Rect, r2: Rect) -> bool {
    return r1.x< r2.x + r2.w
    && r1.x + r1.w > r2.x
    && r1.y < r2.y + r2.h
    && r1.y + r1.h > r2.y;
}

pub fn increment_or_zero(num: i32, max: i32) -> i32 {
    if num + 1 > max { 0 } else { num + 1}
}

pub fn draw_texture_sized(texture: &Texture2D, x: f32, y: f32, color: Color, size: f32, rot: f32) {
    draw_texture_ex(texture, x, y, color, DrawTextureParams { dest_size: Some( Vec2 { x: size, y: size}), rotation: rot, ..Default::default()});
}
