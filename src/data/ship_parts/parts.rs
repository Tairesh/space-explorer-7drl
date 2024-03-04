use serde::{Deserialize, Serialize};

use crate::colors;
use crate::ui::Tile;

use super::{Activation, ShipPartInteract, ShipPartView};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Frame {
    hp: u32,
}

impl Frame {
    pub fn new() -> Self {
        Frame { hp: 100 }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipPartView for Frame {
    fn z_index(&self) -> i8 {
        0
    }

    fn tile(&self) -> Tile {
        Tile::new('┼', colors::LIGHT_GRAY, None)
    }
}

impl ShipPartInteract for Frame {
    fn is_walkable(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub enum WingSegment {
    Normal,
    LeftFront,
    RightFront,
    LeftBack,
    RightBack,
}

impl From<WingSegment> for char {
    fn from(var: WingSegment) -> Self {
        match var {
            WingSegment::Normal => 'M',
            WingSegment::LeftFront => 'd',
            WingSegment::RightFront => 'b',
            WingSegment::LeftBack => 'V',
            WingSegment::RightBack => 'P',
        }
    }
}

impl From<&str> for WingSegment {
    fn from(ch: &str) -> Self {
        match ch {
            "d" => WingSegment::LeftFront,
            "b" => WingSegment::RightFront,
            "M" => WingSegment::Normal,
            "V" => WingSegment::LeftBack,
            "P" => WingSegment::RightBack,
            _ => unimplemented!("Invalid wing segment: '{}'", ch),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Wing {
    hp: u32,
    var: WingSegment,
}

impl Wing {
    pub fn new(ch: &str) -> Self {
        Self {
            hp: 30,
            var: ch.into(),
        }
    }
}

impl Default for Wing {
    fn default() -> Self {
        Self::new("M")
    }
}

impl ShipPartView for Wing {
    fn z_index(&self) -> i8 {
        1
    }

    fn tile(&self) -> Tile {
        Tile::new(self.var.into(), colors::LIGHT_GRAY, None)
    }
}

impl ShipPartInteract for Wing {
    fn is_walkable(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
pub enum WallSegment {
    Vertical,
    Horizontal,
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
    VerticalLeft,
    VerticalRight,
    HorizontalTop,
    HorizontalBottom,
    Cross,
}

impl From<WallSegment> for char {
    fn from(var: WallSegment) -> Self {
        match var {
            WallSegment::Vertical => '║',
            WallSegment::Horizontal => '═',
            WallSegment::LeftTop => '╔',
            WallSegment::LeftBottom => '╚',
            WallSegment::RightTop => '╗',
            WallSegment::RightBottom => '╝',
            WallSegment::VerticalLeft => '╠',
            WallSegment::VerticalRight => '╣',
            WallSegment::HorizontalTop => '╦',
            WallSegment::HorizontalBottom => '╩',
            WallSegment::Cross => '╬',
        }
    }
}

impl From<&str> for WallSegment {
    fn from(ch: &str) -> Self {
        match ch {
            "═" => WallSegment::Horizontal,
            "║" => WallSegment::Vertical,
            "╔" => WallSegment::LeftTop,
            "╚" => WallSegment::LeftBottom,
            "╗" => WallSegment::RightTop,
            "╝" => WallSegment::RightBottom,
            "╠" => WallSegment::VerticalLeft,
            "╣" => WallSegment::VerticalRight,
            "╦" => WallSegment::HorizontalTop,
            "╩" => WallSegment::HorizontalBottom,
            "╬" => WallSegment::Cross,
            _ => unimplemented!("Invalid wall segment: '{}'", ch),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Wall {
    hp: u32,
    var: WallSegment,
}

impl Wall {
    pub fn new(ch: &str) -> Self {
        Self {
            hp: 100,
            var: ch.into(),
        }
    }
}

impl Default for Wall {
    fn default() -> Self {
        Self::new("╬")
    }
}

impl ShipPartView for Wall {
    fn z_index(&self) -> i8 {
        2
    }

    fn tile(&self) -> Tile {
        Tile::with_floor(self.var.into(), colors::LIGHT_STEEL_BLUE)
    }

    fn is_transparent(&self) -> bool {
        false
    }
}

impl ShipPartInteract for Wall {
    fn is_walkable(&self) -> bool {
        false
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Roof {
    hp: u32,
}

impl Roof {
    pub fn new() -> Self {
        Self { hp: 50 }
    }
}

impl Default for Roof {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipPartView for Roof {
    fn z_index(&self) -> i8 {
        100
    }

    fn is_roof(&self) -> bool {
        true
    }

    fn tile(&self) -> Tile {
        Tile::with_floor('+', colors::LIGHT_STEEL_BLUE)
    }
}

impl ShipPartInteract for Roof {
    fn is_walkable(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Floor {
    hp: u32,
}

impl Floor {
    pub fn new() -> Self {
        Self { hp: 20 }
    }
}

impl Default for Floor {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipPartView for Floor {
    fn z_index(&self) -> i8 {
        1
    }

    fn tile(&self) -> Tile {
        Tile::with_floor('.', colors::STEEL_GRAY)
    }
}

impl ShipPartInteract for Floor {
    fn is_walkable(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Door {
    hp: u32,
    open: bool,
    locked: bool,
}

impl Door {
    pub fn new(open: bool, locked: bool) -> Self {
        Self {
            hp: 42,
            open,
            locked,
        }
    }
}

impl Default for Door {
    fn default() -> Self {
        Self::new(false, false)
    }
}

impl ShipPartView for Door {
    fn z_index(&self) -> i8 {
        2
    }

    fn tile(&self) -> Tile {
        if self.open {
            Tile::with_floor('.', colors::LIGHT_STEEL_BLUE)
        } else if self.locked {
            Tile::new('≡', colors::LIGHT_STEEL_BLUE, Some(colors::ORANGE_RED))
        } else {
            Tile::with_floor('≡', colors::LIGHT_STEEL_BLUE)
        }
    }

    fn is_transparent(&self) -> bool {
        self.open
    }
}

impl ShipPartInteract for Door {
    fn is_walkable(&self) -> bool {
        self.open
    }

    fn supported_actions(&self) -> &[Activation] {
        if self.open {
            &[Activation::Close]
        } else if self.locked {
            &[Activation::Open, Activation::Unlock]
        } else {
            &[Activation::Open, Activation::Lock]
        }
    }

    fn act(&mut self, action: Activation) {
        // TODO: log
        match action {
            Activation::Open => {
                if self.locked {
                    println!("Door is locked!");
                } else {
                    self.open = true;
                }
            }
            Activation::Close => {
                self.open = false;
            }
            Activation::Unlock => {
                self.locked = false;
            }
            Activation::Lock => {
                self.locked = true;
            }
            _ => {}
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Seat {
    hp: u32,
    // some other data
}

impl Seat {
    pub fn new() -> Self {
        Self { hp: 10 }
    }
}

impl Default for Seat {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipPartView for Seat {
    fn z_index(&self) -> i8 {
        9
    }

    fn tile(&self) -> Tile {
        Tile::new('▬', colors::STEEL_GRAY, Some(colors::DARK_GRAY))
    }
}

impl ShipPartInteract for Seat {
    fn is_walkable(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Terminal {
    hp: u32,
    // some other data
}

impl Terminal {
    pub fn new() -> Self {
        Self { hp: 10 }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

impl ShipPartView for Terminal {
    fn z_index(&self) -> i8 {
        10
    }

    fn tile(&self) -> Tile {
        Tile::new('◘', colors::STEEL_GRAY, Some(colors::LIME_GREEN))
    }
}

impl ShipPartInteract for Terminal {
    fn is_walkable(&self) -> bool {
        false
    }

    fn supported_actions(&self) -> &[Activation] {
        &[Activation::UseTerminal]
    }
}
