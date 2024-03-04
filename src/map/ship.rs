use geometry::Point;
use serde::{Deserialize, Serialize};

use crate::data::ship_class::ShipClass;
use crate::data::ship_parts::{Activation, ShipPart, ShipPartInteract, ShipPartView};

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

    pub fn passage(&self) -> bool {
        self.top_part().map(ShipPart::passage).unwrap_or(true)
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

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ship {
    pub name: String,
    pub class_name: String,
    pub tiles: Vec<ShipTile>,
    pub bounds: (i32, i32),
    // pub squawk: Squawk,  // TODO: implement squawk code (as Part)
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

// impl FovMap for Ship {
//     fn dimensions(&self) -> Point {
//         self.bounds.into()
//     }
//
//     fn is_opaque(&self, idx: usize) -> bool {
//         !self.tiles[idx].is_transparent()
//     }
// }
