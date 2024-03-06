use geometry::Vec2;
use tetra::graphics::text::{Font, Text};
use tetra::graphics::{Color, DrawParams};
use tetra::Context;

use super::{UiPosition, UiSprite};

#[derive(Debug)]
pub struct Label {
    text: Text,
    color: Color,
    position_set: UiPosition,
    position_calculated: Option<Vec2>,
}

impl Label {
    pub fn new(font: Font, text: impl Into<String>, color: Color, position: UiPosition) -> Self {
        let text = Text::new(text, font);
        Self {
            text,
            color,
            position_set: position,
            position_calculated: None,
        }
    }
}

impl UiSprite for Label {
    fn positionate(&mut self, ctx: &mut Context, window_size: (i32, i32)) {
        let bounds = self.text.get_bounds(ctx).unwrap();
        self.position_calculated = Some(
            self.position_set
                .calc((bounds.width, bounds.height), window_size),
        );
    }

    fn draw(&mut self, ctx: &mut Context) {
        let params = DrawParams::new()
            .position(self.position_calculated.unwrap())
            .color(self.color);
        self.text.draw(ctx, params);
    }

    fn as_label(&mut self) -> Option<&mut Label> {
        Some(self)
    }
}
