use macroquad::prelude::*;

#[no_mangle]
pub fn get_skip_rect() -> Rect {
    Rect { x: 250.0, y: 750.0, w: 450.0, h: 70.0}
}

#[no_mangle]
pub fn get_reroll_rect() -> Rect {
    Rect { x: 1000.0, y: 750.0, w: 450.0, h: 70.0}
}