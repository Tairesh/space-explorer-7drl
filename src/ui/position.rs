#![allow(dead_code)]

use geometry::Vec2;
use tetra::Context;

#[derive(Debug, Clone, Copy)]
pub enum UiPosition {
    Fixed(Vec2),
    TopCenter {
        margin_top: f32,
    },
    TopCenterByLeft {
        margin_top: f32,
        margin_left: f32,
    },
    TopCenterByRight {
        margin_top: f32,
        margin_right: f32,
    },
    CenterWithMargin {
        margin_vertical: f32,
        margin_horizontal: f32,
    },
}

impl UiPosition {
    pub fn calc(self, ctx: &mut Context, bounds: (f32, f32)) -> Vec2 {
        match self {
            Self::Fixed(pos) => pos,
            Self::TopCenter { margin_top: y } => {
                let (w, _) = tetra::window::get_size(ctx);
                Vec2::new(w as f32 / 2.0 - bounds.0 / 2.0, y)
            }
            Self::TopCenterByLeft {
                margin_top: y,
                margin_left: x,
            } => {
                let (w, _) = tetra::window::get_size(ctx);
                Vec2::new(w as f32 / 2.0 - bounds.0 / 2.0 + x, y)
            }
            Self::TopCenterByRight {
                margin_top: y,
                margin_right: x,
            } => {
                let (w, _) = tetra::window::get_size(ctx);
                Vec2::new(w as f32 / 2.0 - bounds.0 / 2.0 - x, y)
            }
            Self::CenterWithMargin {
                margin_vertical: y,
                margin_horizontal: x,
            } => {
                let (w, h) = tetra::window::get_size(ctx);
                Vec2::new(
                    w as f32 / 2.0 - bounds.0 / 2.0 + x,
                    h as f32 / 2.0 - bounds.1 / 2.0 + y,
                )
            }
        }
    }
}
