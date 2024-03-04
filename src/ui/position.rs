#![allow(dead_code)]

use geometry::Vec2;
use tetra::graphics::Rectangle;
use tetra::Context;

#[derive(Debug, Clone, Copy)]
pub enum UiPosition {
    Fixed(Vec2),
    TopCenter { margin_top: f32 },
    TopCenterByLeft { margin_top: f32, margin_left: f32 },
    TopCenterByRight { margin_top: f32, margin_right: f32 },
}

impl UiPosition {
    pub fn calc(self, ctx: &mut Context, bounds: Rectangle) -> Vec2 {
        match self {
            UiPosition::Fixed(pos) => pos,
            UiPosition::TopCenter { margin_top: y } => {
                let (w, _) = tetra::window::get_size(ctx);
                Vec2::new(w as f32 / 2.0 - bounds.width / 2.0, y)
            }
            UiPosition::TopCenterByLeft {
                margin_top: y,
                margin_left: x,
            } => {
                let (w, _) = tetra::window::get_size(ctx);
                Vec2::new(w as f32 / 2.0 - bounds.width / 2.0 + x, y)
            }
            UiPosition::TopCenterByRight {
                margin_top: y,
                margin_right: x,
            } => {
                let (w, _) = tetra::window::get_size(ctx);
                Vec2::new(w as f32 / 2.0 - bounds.width / 2.0 - x, y)
            }
        }
    }
}
