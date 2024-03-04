use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use geometry::Direction;
use tetra::input::{Key, KeyModifier};
use tetra::{input, Context, Event, State};

use crate::assets::Assets;
use crate::data::game_data::GameData;
use crate::game::ChangeState;
use crate::input::get_direction_keys_down;
use crate::map::{Player, Ship};
use crate::ui::ShipView;

#[derive(Debug)]
pub struct ShipWalk {
    ship: Rc<RefCell<Ship>>,
    player: Rc<RefCell<Player>>,
    ship_view: ShipView,
    // assets: Rc<Assets>,
    last_player_move: Instant,
}

impl ShipWalk {
    pub fn new(ctx: &mut Context, game_data: &GameData, assets: Rc<Assets>) -> Self {
        let ship = Rc::new(RefCell::new(Ship::generate(
            "Dugong I",
            game_data.ships.get("dugong").unwrap(),
        )));
        let player = Rc::new(RefCell::new(Player::new(ship.borrow().find_start_point())));
        let ship_view = ShipView::new(ctx, ship.clone(), player.clone(), assets.clone());
        Self {
            ship,
            player,
            ship_view,
            // assets,
            last_player_move: Instant::now(),
        }
    }

    fn try_move_player(&mut self, ctx: &mut Context, direction: Direction) {
        let new_pos = self.player.borrow().pos + direction;
        if self
            .ship
            .borrow()
            .get_tile(new_pos)
            .map_or(false, |tile| tile.is_walkable())
        {
            self.player.borrow_mut().pos = new_pos;
            self.ship_view.update(ctx);
        }
    }
}

impl State for ShipWalk {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        tetra::graphics::clear(ctx, crate::colors::SPACE_VIOLET);
        self.ship_view.draw(ctx);

        Ok(())
    }
}

impl super::State for ShipWalk {
    fn update(&mut self, ctx: &mut Context) -> Option<ChangeState> {
        if let Some(dir) = get_direction_keys_down(ctx) {
            if self.last_player_move.elapsed().as_millis() > 100
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_player_move = Instant::now();
                self.try_move_player(ctx, dir);
            }
        }

        None
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> Option<ChangeState> {
        match event {
            Event::KeyPressed { key: Key::Escape } => return Some(ChangeState::GoToMainMenu),
            Event::KeyPressed { key: Key::Z } => {
                if input::is_key_modifier_down(ctx, KeyModifier::Shift) {
                    self.ship_view.dec_zoom(ctx);
                } else {
                    self.ship_view.inc_zoom(ctx);
                }
            }
            Event::MouseWheelMoved { amount } => {
                if amount.y > 0 {
                    self.ship_view.inc_zoom(ctx);
                } else {
                    self.ship_view.dec_zoom(ctx);
                }
            }
            _ => {}
        }

        None
    }
}
