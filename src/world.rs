use std::cell::RefCell;
use std::rc::Rc;

use crate::data::game_data::GameData;
use crate::map::{Player, Ship};

pub struct World {
    pub player: Rc<RefCell<Player>>,
    pub ship: Rc<RefCell<Ship>>,
}

impl World {
    pub fn new(game_data: &GameData) -> World {
        let ship = Ship::generate("Dugong", game_data.ships.get("dugong").unwrap());
        World {
            player: Rc::new(RefCell::new(Player::new(ship.find_start_point()))),
            ship: Rc::new(RefCell::new(ship)),
        }
    }
}
