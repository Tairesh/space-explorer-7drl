use geometry::Vec2;

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
    pub fn calc(self, bounds: (f32, f32), window_size: (i32, i32)) -> Vec2 {
        match self {
            Self::Fixed(pos) => pos,
            Self::TopCenter { margin_top: y } => {
                Vec2::new(window_size.0 as f32 / 2.0 - bounds.0 / 2.0, y)
            }
            Self::TopCenterByLeft {
                margin_top: y,
                margin_left: x,
            } => Vec2::new(window_size.0 as f32 / 2.0 - bounds.0 / 2.0 + x, y),
            Self::TopCenterByRight {
                margin_top: y,
                margin_right: x,
            } => Vec2::new(window_size.0 as f32 / 2.0 - bounds.0 / 2.0 - x, y),
            Self::CenterWithMargin {
                margin_vertical: y,
                margin_horizontal: x,
            } => Vec2::new(
                window_size.0 as f32 / 2.0 - bounds.0 / 2.0 + x,
                window_size.1 as f32 / 2.0 - bounds.1 / 2.0 + y,
            ),
        }
    }
}
