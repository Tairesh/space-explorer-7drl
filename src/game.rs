use std::rc::Rc;

use tetra::{Context, State};

use crate::assets::Assets;
use crate::states::MainMenu;

#[derive(Debug)]
pub enum GameState {
    MainMenu(MainMenu),
    Exit,
}

impl GameState {
    pub fn draw(&mut self, ctx: &mut Context) {
        match self {
            GameState::MainMenu(main_menu) => {
                main_menu.draw(ctx);
            }
            _ => {}
        }
    }
}

pub struct Game {
    assets: Rc<Assets>,
    state: GameState,
}

impl Game {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let assets = Rc::new(Assets::load(ctx)?);
        Ok(Self {
            state: GameState::MainMenu(MainMenu::new(ctx, assets.clone())?),
            assets,
        })
    }
}

impl State for Game {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match &mut self.state {
            GameState::MainMenu(main_menu) => {
                if let Some(state) = main_menu.update(ctx) {
                    self.state = state;
                }
            }
            GameState::Exit => {
                tetra::window::quit(ctx);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        self.state.draw(ctx);
        Ok(())
    }
}
