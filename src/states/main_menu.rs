use std::rc::Rc;

use tetra::Context;

use crate::assets::Assets;
use crate::colors;
use crate::game::GameState;
use crate::ui::{Label, UiPosition};

#[derive(Debug)]
pub struct MainMenu {
    labels: [Label; 4],
}

impl MainMenu {
    pub fn new(ctx: &mut Context, assets: Rc<Assets>) -> tetra::Result<Self> {
        let logo = Label::new(
            ctx,
            assets.fonts.logo.font.clone(),
            crate::TITLE,
            colors::ORANGE,
            UiPosition::TopCenter { margin_top: 20.0 },
        );
        let version = Label::new(
            ctx,
            assets.fonts.consolab14.font.clone(),
            crate::VERSION,
            colors::ORANGE,
            UiPosition::TopCenterByRight {
                margin_top: 100.0,
                margin_right: 360.0,
            },
        );
        let copyright = Label::new(
            ctx,
            assets.fonts.consolab14.font.clone(),
            "by Tairesh",
            colors::ORANGE,
            UiPosition::TopCenterByLeft {
                margin_top: 100.0,
                margin_left: 350.0,
            },
        );
        let help = Label::new(
            ctx,
            assets.fonts.handel24.font.clone(),
            "Press [Enter] to start, [Q] to exit",
            colors::ORANGE_RED,
            UiPosition::TopCenter { margin_top: 400.0 },
        );
        Ok(Self {
            labels: [logo, version, copyright, help],
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> Option<GameState> {
        if tetra::input::is_key_pressed(ctx, tetra::input::Key::Q) {
            return Some(GameState::Exit);
        }

        None
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        tetra::graphics::clear(ctx, colors::SPACE_VIOLET);
        for label in &mut self.labels {
            label.draw(ctx);
        }
    }
}
