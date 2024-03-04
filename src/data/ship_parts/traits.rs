use std::cmp::Ordering;

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::ui::Tile;

use super::{Activation, Door, Floor, Frame, Roof, Seat, Terminal, Wall, Wing};

#[enum_dispatch(ShipPart)]
pub trait ShipPartView {
    /// only part with MAXIMUM z_index will be displayed
    fn z_index(&self) -> i8;
    /// true if it's a roof (invisible when inside)
    fn is_roof(&self) -> bool {
        false
    }
    /// tile representation
    fn tile(&self) -> Tile;
    /// is tile with this part transparent
    fn is_transparent(&self) -> bool {
        true
    }
}

#[enum_dispatch(ShipPart)]
pub trait ShipPartInteract {
    fn is_walkable(&self) -> bool;
    fn supported_actions(&self) -> &[Activation] {
        &[]
    }
    fn supports_action(&self, activation: Activation) -> bool {
        self.supported_actions().contains(&activation)
    }
    fn act(&mut self, _action: Activation) {}
}

#[enum_dispatch]
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum ShipPart {
    Frame,
    Wing,
    Wall,
    Roof,
    Floor,
    Door,
    Seat,
    Terminal,
}

impl PartialOrd<Self> for ShipPart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShipPart {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z_index().cmp(&other.z_index())
    }
}
