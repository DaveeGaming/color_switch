
// FEATURE CREEP CORNER
// Weapon for diff color ships
// Bullethell kinda stuff


//TODO: Change shooting to be on the mouse
//TODO: Crosshair color for palettes
//TODO: Better difficulty curve
//TODO: Melee attack
//TODO: Particle system
//TODO: Abilities like shield and stuff
//TODO: instead of upgrade, heal or reroll
//TODO: Multiple difficulty levels, 

use assets::Textures;
use camera::mouse;
use macroquad::{conf::Conf, prelude::*};
use crate::game::*;

mod game;
mod enemy;
mod collision;
mod player;
mod collection;
mod log;
mod hp;
mod particle;
mod bullet;
mod colors;
mod characters;
mod upgrade;
mod assets;
mod options;
mod wave;
mod menu;

// #[hot_lib_reloader::hot_module(dylib = "lib")]
// mod hot_lib {
//     use macroquad::prelude::*;
//     hot_functions_from_file!("lib/src/lib.rs");
// }

use crate::log::*;
use miniquad::{conf::Platform, window::screen_size};

pub fn config() -> Conf {
    Conf { 
        miniquad_conf: 
            miniquad::conf::Conf {
                platform: Platform {
                    swap_interval: Some(0),
                    ..Default::default()
                },
                window_title: String::from("Color swap"),
                ..Default::default()
            }, 
            ..Default::default()
        }
    }
    
    

    
#[macroquad::main(config)]
async fn main() {
    let canvas = render_target(DESIGN_WIDTH as u32, DESIGN_HEIGHT as u32);
    canvas.texture.set_filter(FilterMode::Nearest);
    let mut camera = Camera2D::from_display_rect(Rect {
        x: 0.,
        y: 0.,
        w: (DESIGN_WIDTH) as f32,
        h: (DESIGN_HEIGHT) as f32   
    });
    camera.zoom.y = -camera.zoom.y;
    camera.render_target = Some(canvas.clone());


    let (screen_w, screen_h) = screen_size();
    let scale = f32::min(screen_w / DESIGN_WIDTH, screen_h / DESIGN_HEIGHT);
    let x_center = (screen_w - DESIGN_WIDTH * scale) / 2.0;
    let y_center = (screen_h - DESIGN_HEIGHT * scale) / 2.0;
    let texture = Texture2D::from_file_with_format( include_bytes!("../assets/loading.png"), None);
    set_camera(&camera);
    draw_texture(&texture, 0.0, 0.0, WHITE);
    
    set_default_camera();
    clear_background(BLANK);
    draw_texture_ex(&canvas.texture, x_center, y_center, WHITE, 
        DrawTextureParams {
            dest_size: Some( Vec2 { x: DESIGN_WIDTH * scale, y: DESIGN_HEIGHT * scale  }),
            ..Default::default()
        }
    );
    next_frame().await;

    log(&"Loading texture created");
    log(&"Starting to load game state");
    let mut game = Game::default().await;
    log(&"Finished loading game state");
    let time = miniquad::date::now();

    // Browser storage handling
    let storage = &mut quad_storage::STORAGE.lock().unwrap();    

    
    let highscore = storage.get("highscore");
    if highscore.is_none() {
        storage.set("highscore", &0.to_string());
    } else {
        let highscore = highscore.unwrap();
        game.high_score = highscore.parse::<i32>().unwrap();
    }

    let sound = storage.get("sound_volume");
    if sound.is_none() {
        storage.set("sound_volume", &3.to_string());
    } else {
        let sound = sound.unwrap();
        game.music_level = sound.parse::<i32>().unwrap();
    }

    let effect = storage.get("effect_volume");
    if effect.is_none() {
        storage.set("effect_volume", &3.to_string());
    } else {
        let effect = effect.unwrap();
        game.effect_level = effect.parse::<i32>().unwrap();
    }

    let orangeyellow = storage.get("orangeyellow");
    if orangeyellow.is_none() {
        storage.set("orangeyellow", &false.to_string());
    } else {
        let orangeyellow = orangeyellow.unwrap();
        game.unlocks.orangegreen = orangeyellow.parse::<bool>().unwrap();
    }
    
    let purpleyellow = storage.get("purpleyellow");
    if purpleyellow.is_none() {
        storage.set("purpleyellow", &false.to_string());
    } else {
        let purpleyellow = purpleyellow.unwrap();
        game.unlocks.purpleyellow = purpleyellow.parse::<bool>().unwrap();
    }
    macroquad::rand::srand(time as u64);
    draw_text_centered_c(&"0", DESIGN_WIDTH/2.0, 150.0, 260.0, &game.assets.font_monogram, WHITE);
    draw_text_centered_c(&"1", DESIGN_WIDTH/2.0, 150.0, 260.0, &game.assets.font_monogram, WHITE);
    draw_text_centered_c(&"2", DESIGN_WIDTH/2.0, 150.0, 260.0, &game.assets.font_monogram, WHITE);
    draw_text_centered_c(&"3", DESIGN_WIDTH/2.0, 150.0, 260.0, &game.assets.font_monogram, WHITE);
    draw_text_centered_c(&"4", DESIGN_WIDTH/2.0, 150.0, 260.0, &game.assets.font_monogram, WHITE);
    draw_text_centered_c(&"5", DESIGN_WIDTH/2.0, 150.0, 260.0, &game.assets.font_monogram, WHITE);
    draw_text_centered_c(&"6", DESIGN_WIDTH/2.0, 150.0, 260.0, &game.assets.font_monogram, WHITE);
    draw_text_centered_c(&"7", DESIGN_WIDTH/2.0, 150.0, 260.0, &game.assets.font_monogram, WHITE);
    draw_text_centered_c(&"8", DESIGN_WIDTH/2.0, 150.0, 260.0, &game.assets.font_monogram, WHITE);
    draw_text_centered_c(&"9", DESIGN_WIDTH/2.0, 150.0, 260.0, &game.assets.font_monogram, WHITE);
    
    

    loop {
        
        if is_key_down(KeyCode::LeftShift) && is_key_pressed(KeyCode::R) {
            warn!("Reloading textures!");
            game.assets.t = Textures::new().await;
        }
        
        
        let (screen_w, screen_h) = screen_size();
        let scale = f32::min(screen_w / DESIGN_WIDTH, screen_h / DESIGN_HEIGHT);
        let x_center = (screen_w - DESIGN_WIDTH * scale) / 2.0;
        let y_center = (screen_h - DESIGN_HEIGHT * scale) / 2.0;
        

        let mut mouse_dir = mouse_position_local();
        mouse_dir.x = (mouse_dir.x + 1.0) / 2.0;
        mouse_dir.y = (mouse_dir.y + 1.0) / 2.0;

        mouse_dir.x *= DESIGN_WIDTH;
        mouse_dir.y *= DESIGN_HEIGHT;

        game.mouse_pos = mouse_dir;

        set_camera(&camera);
        game.update();
        game.draw();
        game.save_data(storage);
        
        set_default_camera();
        clear_background(BLANK);
        draw_texture_ex(&canvas.texture, x_center, y_center, WHITE, 
            DrawTextureParams {
                dest_size: Some( Vec2 { x: DESIGN_WIDTH * scale, y: DESIGN_HEIGHT * scale  }),
                ..Default::default()
            }
        );
        next_frame().await
    }
}
