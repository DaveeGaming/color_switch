use macroquad::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ColorState {
    Primary,
    Secondary
}

impl ColorState {
    pub fn next(&self) -> Self {
        match self {
            Self::Secondary => Self::Primary,
            Self::Primary => Self::Secondary,
        }
    }
}
#[derive(Clone, Copy)]
pub struct ColorPalette {
    pub fg_primary: Color, 
    pub fg_secondary: Color,
    pub bg_primary: Color,
    pub bg_secondary: Color,
}

impl ColorPalette {
    pub fn default() -> Self {
        ColorPalette {
            fg_primary: BLUE,
            fg_secondary: RED,
            bg_primary: Color{ r: 0.0, g: 0.0, b: 0.1, a: 1.0},
            bg_secondary: Color{ r: 0.1, g: 0.0, b: 0.0, a: 1.0},
        }
    }

    pub fn create_from(primary: Color, secondary: Color) -> Self {
        ColorPalette {
            fg_primary: primary,
            fg_secondary: secondary,
            bg_primary: Color{ r: primary.r * 0.08, g: primary.g * 0.08, b: primary.b * 0.08, a: 1.0},
            bg_secondary: Color{ r: secondary.r * 0.08, g: secondary.g * 0.08, b: secondary.b * 0.08, a: 1.0},
        }
    }
}