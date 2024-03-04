use std::rc::Rc;

use tetra::{Context, Event, TetraError};

use crate::assets::Assets;
use crate::data::game_data::GameData;
use crate::states::{MainMenu, ShipWalk, State};

#[derive(Debug)]
pub enum ChangeState {
    GoToMainMenu,
    LoadShipWalk,
    Exit,
}

pub struct Game {
    data: Rc<GameData>,
    assets: Rc<Assets>,
    state: Box<dyn State>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let data = Rc::new(GameData::load());
        let assets = Rc::new(Assets::load(ctx)?);
        Ok(Self {
            state: Box::new(MainMenu::new(ctx, &assets)),
            data,
            assets,
        })
    }

    fn change_state(&mut self, ctx: &mut Context, result: ChangeState) {
        self.state = match result {
            ChangeState::GoToMainMenu => Box::new(MainMenu::new(ctx, &self.assets)),
            ChangeState::LoadShipWalk => {
                Box::new(ShipWalk::new(ctx, &self.data, self.assets.clone()))
            }
            ChangeState::Exit => std::process::exit(0),
        };
    }
}

impl tetra::State for Game {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if let Some(result) = State::update(self.state.as_mut(), ctx) {
            self.change_state(ctx, result);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        self.state.draw(ctx)
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), TetraError> {
        if let Some(result) = State::event(self.state.as_mut(), ctx, event) {
            self.change_state(ctx, result);
        }

        Ok(())
    }
}
