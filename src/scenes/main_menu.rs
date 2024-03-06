use tetra::input::Key;
use tetra::{Context, Event};

use crate::assets::Assets;
use crate::colors;
use crate::ui::{Label, UiPosition, UiSprite};

use super::{Scene, SceneEvent, SceneImpl};

pub struct MainMenu {
    sprites: [Box<dyn UiSprite>; 4],
}

impl MainMenu {
    pub fn new(assets: &Assets) -> Self {
        let logo = Label::new(
            assets.fonts.logo.font.clone(),
            crate::TITLE,
            colors::ORANGE,
            UiPosition::TopCenter { margin_top: 20.0 },
        );
        let version = Label::new(
            assets.fonts.consolab14.font.clone(),
            crate::VERSION,
            colors::ORANGE,
            UiPosition::TopCenterByRight {
                margin_top: 100.0,
                margin_right: 360.0,
            },
        );
        let copyright = Label::new(
            assets.fonts.consolab14.font.clone(),
            "by Tairesh",
            colors::ORANGE,
            UiPosition::TopCenterByLeft {
                margin_top: 100.0,
                margin_left: 350.0,
            },
        );
        let help = Label::new(
            assets.fonts.handel24.font.clone(),
            "Press [Enter] to start, [Q] to exit",
            colors::ORANGE_RED,
            UiPosition::TopCenter { margin_top: 400.0 },
        );
        Self {
            sprites: [
                Box::new(logo),
                Box::new(version),
                Box::new(copyright),
                Box::new(help),
            ],
        }
    }
}

impl SceneImpl for MainMenu {
    fn on_event(&mut self, _ctx: &mut Context, event: Event) -> Option<SceneEvent> {
        match event {
            Event::KeyPressed { key: Key::Q } => Some(SceneEvent::Exit),
            Event::KeyPressed { key: Key::Enter } => Some(SceneEvent::ChangeScene(Scene::ShipWalk)),
            _ => None,
        }
    }

    fn before_draw(&mut self, ctx: &mut Context) {
        tetra::graphics::clear(ctx, colors::SPACE_VIOLET);
    }

    fn ui_sprites(&self) -> Option<&[Box<dyn UiSprite>]> {
        Some(&self.sprites)
    }

    fn ui_sprites_mut(&mut self) -> Option<&mut [Box<dyn UiSprite>]> {
        Some(&mut self.sprites)
    }
}
