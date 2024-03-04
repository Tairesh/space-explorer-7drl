use std::cell::RefCell;
use std::rc::Rc;

use bracket_pathfinding::prelude::field_of_view_set;
use geometry::{Point, Vec2};
use tetra::graphics::mesh::{GeometryBuilder, Mesh, ShapeStyle};
use tetra::graphics::{Canvas, DrawParams, Rectangle};
use tetra::{graphics, Context};

use crate::assets::{Assets, TileSet};
use crate::colors;
use crate::map::{Player, Ship};

use super::{Tile, Zoom};

#[derive(Debug)]
pub struct ShipView {
    canvas: Canvas,
    assets: Rc<Assets>,
    player: Rc<RefCell<Player>>,
    ship: Rc<RefCell<Ship>>,
    zoom: Zoom,
}

impl ShipView {
    pub fn new(
        ctx: &mut Context,
        ship: Rc<RefCell<Ship>>,
        player: Rc<RefCell<Player>>,
        assets: Rc<Assets>,
    ) -> Self {
        let canvas = draw_ship(ctx, &ship.borrow(), &player.borrow(), &assets.tileset);
        let zoom = Zoom::default();
        Self {
            canvas,
            assets,
            player,
            ship,
            zoom,
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let (w, h) = tetra::window::get_size(ctx);
        let mut pos = Vec2::new(w as f32 / 2.0, h as f32 / 2.0);
        pos -= Vec2::new(
            TileSet::TILE_SIZE.0 as f32 / 2.0,
            TileSet::TILE_SIZE.1 as f32 / 2.0,
        ) * self.zoom.asf32();
        pos -= Vec2::from(self.player.borrow().pos * TileSet::TILE_SIZE) * self.zoom.asf32();
        self.canvas.draw(
            ctx,
            DrawParams::new().position(pos).scale(self.zoom.as_scale()),
        );
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.canvas = draw_ship(
            ctx,
            &self.ship.borrow(),
            &self.player.borrow(),
            &self.assets.tileset,
        );
    }

    pub fn inc_zoom(&mut self, ctx: &mut Context) {
        self.zoom.inc();
        self.update(ctx);
    }

    pub fn dec_zoom(&mut self, ctx: &mut Context) {
        self.zoom.dec();
        self.update(ctx);
    }
}

fn draw_ship(ctx: &mut Context, ship: &Ship, player: &Player, tileset: &TileSet) -> Canvas {
    let canvas_size = (
        TileSet::TILE_SIZE.0 * ship.bounds.0,
        TileSet::TILE_SIZE.1 * ship.bounds.1,
    );
    let canvas = Canvas::new(ctx, canvas_size.0, canvas_size.1).unwrap();
    graphics::set_canvas(ctx, &canvas);

    let mut bg_builder = GeometryBuilder::new();
    let bg_color = colors::SPACE_VIOLET;
    for (i, tile) in ship.tiles.iter().enumerate() {
        if tile.is_void() {
            continue;
        }
        let tile: Tile = tile.into();
        if let Some(bg) = tile.bg {
            if bg == bg_color {
                let point = Point::from_index(i, ship.bounds.0);
                let pos = Vec2::from(point * TileSet::TILE_SIZE);
                bg_builder
                    .rectangle(
                        ShapeStyle::Fill,
                        Rectangle::new(
                            pos.x,
                            pos.y,
                            TileSet::TILE_SIZE.0 as f32,
                            TileSet::TILE_SIZE.1 as f32,
                        ),
                    )
                    .ok();
            }
        }
    }
    let bg_mesh = bg_builder.build_mesh(ctx).unwrap();
    bg_mesh.draw(ctx, DrawParams::new().color(bg_color));

    let mesh = Mesh::rectangle(
        ctx,
        ShapeStyle::Fill,
        Rectangle::new(
            0.0,
            0.0,
            TileSet::TILE_SIZE.0 as f32,
            TileSet::TILE_SIZE.1 as f32,
        ),
    )
    .unwrap();

    let braket_point = bracket_pathfinding::prelude::Point::new(player.pos.x, player.pos.y);
    let fov = field_of_view_set(braket_point, ship.bounds.0.max(ship.bounds.1), ship);
    let mut fov_builder = GeometryBuilder::new();
    for (i, tile) in ship.tiles.iter().enumerate() {
        if tile.is_void() {
            continue;
        }
        let tile: Tile = tile.into();
        let point = Point::from_index(i, ship.bounds.0);
        let braket_point = bracket_pathfinding::prelude::Point::new(point.x, point.y);
        let pos = Vec2::from(point * TileSet::TILE_SIZE);
        if let Some(color) = tile.bg {
            if !fov.contains(&braket_point) {
                fov_builder
                    .rectangle(
                        ShapeStyle::Fill,
                        Rectangle::new(
                            pos.x,
                            pos.y,
                            TileSet::TILE_SIZE.0 as f32,
                            TileSet::TILE_SIZE.1 as f32,
                        ),
                    )
                    .ok();
            }
            if color != bg_color {
                mesh.draw(ctx, DrawParams::new().position(pos).color(color));
            }
        }
        if player.pos == point {
            tileset.draw(
                ctx,
                '@',
                DrawParams::new().position(pos).color(colors::WHITE),
            );
        } else {
            tileset.draw(ctx, tile.ch, DrawParams::new().position(pos).color(tile.fg));
        }
    }

    let mesh = fov_builder.build_mesh(ctx).unwrap();
    mesh.draw(ctx, DrawParams::new().color(colors::BLACK.with_alpha(0.5)));

    graphics::reset_canvas(ctx);
    canvas
}
