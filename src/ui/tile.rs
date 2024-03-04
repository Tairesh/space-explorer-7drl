use tetra::graphics::Color;

use crate::colors;

#[derive(Debug, Clone)]
pub struct Tile {
    pub ch: char,
    pub fg: Color,
    pub bg: Option<Color>,
}

impl Tile {
    pub fn new(ch: char, fg: Color, bg: Option<Color>) -> Self {
        Self { ch, fg, bg }
    }

    pub fn white(ch: char) -> Self {
        Self {
            ch,
            fg: Color::WHITE,
            bg: None,
        }
    }

    pub fn with_floor(ch: char, fg: Color) -> Self {
        Self {
            ch,
            fg,
            bg: Some(colors::DARKEST_GRAY),
        }
    }

    pub fn empty() -> Self {
        Self::white(' ')
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::empty()
    }
}
