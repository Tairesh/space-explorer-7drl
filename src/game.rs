use tetra::input::Key;
use tetra::{graphics, input, Context, State};

use crate::colors;

enum GameState {
    MainMenu,
    Exit,
}

pub struct Game {
    state: GameState,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            state: GameState::MainMenu,
        })
    }
}

impl State for Game {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.state {
            GameState::MainMenu => {
                if input::is_key_pressed(ctx, Key::Q) {
                    self.state = GameState::Exit;
                }
            }
            GameState::Exit => {
                tetra::window::quit(ctx);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, colors::SPACE_VIOLET);
        Ok(())
    }
}
