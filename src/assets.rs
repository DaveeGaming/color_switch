use macroquad::{audio::{load_sound_from_bytes, Sound}, logging::error, text::{load_ttf_font_from_bytes, Font}, texture::Texture2D};
use macroquad_utils::*;

pub struct Assets {
    pub font_monogram: Font,
    pub t: Textures,
    pub menu_song: Sound,
    pub play_song: Sound,
    pub menu_switch: Sound,
    pub shoot: Sound,
    pub hit: Sound,
    pub dead: Sound,
}

#[derive(TextureDynLoader)]
pub struct Textures {
    pub upgrade_frame: Texture2D,
    pub maxhp: Texture2D,
    pub projectile: Texture2D,
    pub size: Texture2D,
    pub slowdmg: Texture2D,
    pub speed: Texture2D,
    pub shooter: Texture2D,
    pub tower: Texture2D,
    pub menu1: Texture2D,
    pub menu2: Texture2D,
    pub garry: Texture2D,
    pub john: Texture2D,
    pub border: Texture2D,
    pub mark: Texture2D,
    pub hpbar: Texture2D,
    pub locked: Texture2D,
    pub bobbobbob: Texture2D,
    pub controls: Texture2D,
    pub reroll: Texture2D,
    pub skip: Texture2D,
    pub bg1: Texture2D,
}

impl Textures {
    pub async fn reload(&mut self) {
        *self = Self::new().await;
    }
}


impl Assets {
    pub async fn default() -> Self {
        let font = load_ttf_font_from_bytes( include_bytes!("..\\assets\\FatPixelFont.ttf") );
        if font.is_err() {
            error!("Unable to load monogram font!")
        }

        return Assets {
            font_monogram: font.unwrap(),
            t: Textures::new().await,
            play_song: load_sound_from_bytes( include_bytes!("..\\assets\\medium_boss.wav") ).await.unwrap(),
            menu_song: load_sound_from_bytes( include_bytes!("..\\assets\\little_slime.wav") ).await.unwrap(),
            menu_switch: load_sound_from_bytes( include_bytes!("..\\assets\\menu.wav") ).await.unwrap(),
            shoot: load_sound_from_bytes( include_bytes!("..\\assets\\shoot.wav") ).await.unwrap(),
            hit: load_sound_from_bytes( include_bytes!("..\\assets\\hit.wav") ).await.unwrap(),
            dead: load_sound_from_bytes( include_bytes!("..\\assets\\dead.wav") ).await.unwrap(),
        }
    }
}