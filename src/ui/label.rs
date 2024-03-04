use tetra::graphics::text::{Font, Text};
use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::Context;

use super::UiPosition;

#[derive(Debug)]
pub struct Label {
    text: Text,
    color: Color,
    position: UiPosition,
    bounds: Rectangle,
}

impl Label {
    pub fn new(
        ctx: &mut Context,
        font: Font,
        text: impl Into<String>,
        color: Color,
        position: UiPosition,
    ) -> Self {
        let mut text = Text::new(text, font);
        let bounds = text.get_bounds(ctx).unwrap();
        Self {
            text,
            color,
            position,
            bounds,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let params = DrawParams::new()
            .position(self.position.calc(ctx, self.bounds))
            .color(self.color);
        self.text.draw(ctx, params);
    }
}
