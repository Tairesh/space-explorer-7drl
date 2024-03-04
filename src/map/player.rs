use geometry::Point;
use serde::{Deserialize, Serialize};

use crate::data::ship_parts::Activation;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub pos: Point,
    pub action: Option<Activation>,
}

impl Player {
    pub fn new(pos: Point) -> Self {
        Player { pos, action: None }
    }
}
