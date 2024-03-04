use tetra::graphics::ImageData;
use tetra::{
    window::{self, WindowPosition::Centered},
    ContextBuilder,
};

use crate::game::Game;

mod assets;
mod colors;
mod cp437;
mod game;
mod states;
mod ui;

const TITLE: &str = "Space Explorer 7DRL";
const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

fn main() -> tetra::Result {
    let mut ctx_builder = ContextBuilder::new(format!("{} {}", TITLE, VERSION), 1366, 768);
    ctx_builder
        .show_mouse(true)
        .vsync(true)
        .key_repeat(true)
        .resizable(true);
    let mut ctx = ctx_builder.build()?;

    let mut icon = ImageData::from_encoded(include_bytes!("../inc/img/icon.png"))?;
    window::set_icon(&mut ctx, &mut icon)?;
    window::set_minimum_size(&mut ctx, 1366, 768)?;
    window::set_maximum_size(&mut ctx, 1920, 1280)?;
    let monitor = window::get_current_monitor(&ctx).unwrap_or(0);
    window::set_position(&mut ctx, Centered(monitor), Centered(monitor));

    ctx.run(Game::new)
}
