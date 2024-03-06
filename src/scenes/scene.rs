use tetra::Context;

use crate::ui::UiSprite;

use super::SceneEvent;

pub enum Scene {
    MainMenu,
    ShipWalk,
}

pub trait SceneImpl {
    fn on_update(&mut self, _ctx: &mut Context) -> Option<SceneEvent> {
        None
    }
    fn on_event(&mut self, _ctx: &mut Context, _event: tetra::Event) -> Option<SceneEvent> {
        None
    }
    fn before_draw(&mut self, _ctx: &mut Context) {}
    fn after_draw(&mut self, _ctx: &mut Context) {}
    fn on_open(&mut self, _ctx: &mut Context) {}
    fn on_resize(&mut self, _ctx: &mut Context, _window_size: (i32, i32)) {}
    fn ui_sprites(&self) -> Option<&[Box<dyn UiSprite>]> {
        None
    }
    fn ui_sprites_mut(&mut self) -> Option<&mut [Box<dyn UiSprite>]> {
        None
    }
    fn custom_event(&mut self, _ctx: &mut Context, _event: u8) -> Option<SceneEvent> {
        None
    }

    fn reposition_all_sprites(&mut self, ctx: &mut Context, window_size: (i32, i32)) {
        if let Some(sprites) = self.ui_sprites_mut() {
            sprites
                .iter_mut()
                .for_each(|sprite| sprite.positionate(ctx, window_size));
        }
    }
}
