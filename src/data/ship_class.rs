use serde::Deserialize;

use crate::map::ShipTile;

use super::ship_parts::{Door, Floor, Frame, Roof, Seat, ShipPart, Terminal, Wall, Wing};

#[derive(Debug, Deserialize)]
pub struct ShipClass {
    pub id: String,
    pub name: String,
    pub tiles: Vec<String>,
    pub bounds: (i32, i32),
}

impl From<&str> for ShipTile {
    fn from(s: &str) -> Self {
        if s == " " {
            return ShipTile { parts: vec![] };
        }
        let mut parts: Vec<ShipPart> = vec![Frame::new().into()];
        match s {
            ch @ ("d" | "b" | "M" | "V" | "P") => {
                parts.push(Wing::new(ch).into());
            }
            ch @ ("╔" | "═" | "╗" | "║" | "╝" | "╚" | "╠" | "╦" | "╣" | "╩" | "╬") =>
            {
                parts.push(Wall::new(ch).into());
            }
            "." => {
                parts.push(Floor::new().into());
                parts.push(Roof::new().into());
            }
            "+" => {
                parts.push(Floor::new().into());
                parts.push(Door::new(false, false).into());
                parts.push(Roof::new().into());
            }
            "=" => {
                parts.push(Floor::new().into());
                parts.push(Door::new(false, true).into());
                parts.push(Roof::new().into());
            }
            "@" => {
                parts.push(Floor::new().into());
                parts.push(Terminal::new().into());
                parts.push(Roof::new().into());
            }
            "h" => {
                parts.push(Floor::new().into());
                parts.push(Seat::new().into());
                parts.push(Roof::new().into());
            }
            _ => println!("'{}' is not a valid tile", s),
        }
        ShipTile { parts }
    }
}
