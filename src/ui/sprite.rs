use tetra::Context;

use super::{Label, ShipView};

pub trait UiSprite {
    fn positionate(&mut self, ctx: &mut Context, window_size: (i32, i32));
    fn draw(&mut self, ctx: &mut Context);

    fn as_label(&mut self) -> Option<&mut Label> {
        None
    }
    fn as_ship_view(&mut self) -> Option<&mut ShipView> {
        None
    }
}
