use tetra::Context;

pub use main_menu::MainMenu;
pub use ship_walk::ShipWalk;

use crate::game::ChangeState;

mod main_menu;
mod ship_walk;

pub trait State: tetra::State {
    fn update(&mut self, _ctx: &mut Context) -> Option<ChangeState> {
        None
    }
    fn event(&mut self, _ctx: &mut Context, _event: tetra::Event) -> Option<ChangeState> {
        None
    }
}
