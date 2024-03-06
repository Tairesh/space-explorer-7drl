use bracket_pathfinding::prelude::{Algorithm2D, BaseMap};
use geometry::Point;
use serde::{Deserialize, Serialize};

use crate::data::ship_class::ShipClass;
use crate::data::ship_parts::{Activation, ShipPart, ShipPartInteract, ShipPartView};
use crate::ui::Tile;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShipTile {
    pub parts: Vec<ShipPart>,
}

impl ShipTile {
    pub fn is_void(&self) -> bool {
        self.parts.is_empty()
    }

    pub fn top_part(&self) -> Option<&ShipPart> {
        self.parts.iter().filter(|p| !p.is_roof()).max()
    }

    pub fn is_walkable(&self) -> bool {
        self.top_part()
            .map(ShipPartInteract::is_walkable)
            .unwrap_or(true)
    }

    pub fn supports_action(&self, action: Activation) -> bool {
        self.parts.iter().any(|p| p.supports_action(action))
    }

    pub fn supports_any_action(&self) -> bool {
        self.parts.iter().any(|p| !p.supported_actions().is_empty())
    }

    pub fn act(&mut self, action: Activation) {
        self.parts
            .iter_mut()
            .filter(|p| p.supports_action(action))
            .for_each(|p| {
                p.act(action);
            });
    }

    pub fn is_transparent(&self) -> bool {
        self.parts.iter().all(ShipPart::is_transparent)
    }
}

impl From<&ShipTile> for Tile {
    fn from(tile: &ShipTile) -> Self {
        if let Some(top_part) = tile.top_part() {
            top_part.tile()
        } else {
            Tile::empty()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ship {
    pub name: String,
    pub class_name: String,
    pub tiles: Vec<ShipTile>,
    pub bounds: (i32, i32),
}

impl Ship {
    pub fn generate<S: Into<String>>(name: S, scheme: &ShipClass) -> Self {
        Self {
            name: name.into(),
            class_name: scheme.name.clone(),
            tiles: scheme
                .tiles
                .iter()
                .map(|s| ShipTile::from(s.as_str()))
                .collect(),
            bounds: scheme.bounds,
        }
    }

    pub fn find_start_point(&self) -> Point {
        Point::new(self.bounds.0 / 2, self.bounds.1 / 2)
    }

    pub fn get_tile(&self, point: Point) -> Option<&ShipTile> {
        self.tiles.get(point.to_index(self.bounds.0)?)
    }

    pub fn get_tile_mut(&mut self, point: Point) -> Option<&mut ShipTile> {
        self.tiles.get_mut(point.to_index(self.bounds.0)?)
    }
}

impl BaseMap for Ship {
    fn is_opaque(&self, idx: usize) -> bool {
        !self.tiles[idx].is_transparent()
    }
}

impl Algorithm2D for Ship {
    fn dimensions(&self) -> bracket_pathfinding::prelude::Point {
        self.bounds.into()
    }
}
