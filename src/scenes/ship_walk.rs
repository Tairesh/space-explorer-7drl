use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use geometry::Direction;
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::Rectangle;
use tetra::input::{Key, KeyModifier};
use tetra::{input, Context, Event};

use crate::assets::{Assets, TileSet};
use crate::data::ship_parts::Activation;
use crate::input::get_direction_keys_down;
use crate::ui::{ShipView, UiSprite};
use crate::world::World;

use super::{Scene, SceneEvent, SceneImpl};

pub struct ShipWalk {
    sprites: Vec<Box<dyn UiSprite>>,
    world: Rc<RefCell<World>>,
    last_player_move: Instant,
    cursor: Mesh,
}

impl ShipWalk {
    pub fn new(ctx: &mut Context, assets: Rc<Assets>, world: Rc<RefCell<World>>) -> Self {
        let ship_view = ShipView::new(
            ctx,
            world.borrow().ship.clone(),
            world.borrow().player.clone(),
            assets.clone(),
        );
        let cursor = Mesh::rectangle(
            ctx,
            ShapeStyle::Stroke(1.0),
            Rectangle::new(
                0.0,
                0.0,
                TileSet::TILE_SIZE.0 as f32,
                TileSet::TILE_SIZE.1 as f32,
            ),
        )
        .unwrap();
        Self {
            sprites: vec![Box::new(ship_view)],
            world,
            last_player_move: Instant::now(),
            cursor,
        }
    }

    fn ship_view(&mut self) -> &mut ShipView {
        self.sprites[0].as_ship_view().unwrap()
    }

    fn try_move_player(&mut self, ctx: &mut Context, direction: Direction) {
        let world = self.world.borrow();
        let new_pos = world.player.borrow().pos + direction;
        let mut ship = world.ship.borrow_mut();
        let tile = ship.get_tile_mut(new_pos);
        if let Some(tile) = tile {
            if tile.is_walkable() {
                world.player.borrow_mut().pos = new_pos;
            } else if tile.supports_action(Activation::Open) {
                tile.act(Activation::Open);
            }
        }
        drop(ship);
        drop(world);
        self.ship_view().update(ctx);
    }
}

impl SceneImpl for ShipWalk {
    fn before_draw(&mut self, ctx: &mut Context) {
        tetra::graphics::clear(ctx, crate::colors::SPACE_VIOLET);
    }

    fn on_update(&mut self, ctx: &mut Context) -> Option<SceneEvent> {
        if let Some(dir) = get_direction_keys_down(ctx) {
            if self.last_player_move.elapsed().as_millis() > 100
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_player_move = Instant::now();
                self.try_move_player(ctx, dir);
            }
        }

        None
    }

    fn on_event(&mut self, ctx: &mut Context, event: Event) -> Option<SceneEvent> {
        match event {
            Event::KeyPressed { key: Key::Escape } => {
                return Some(SceneEvent::ChangeScene(Scene::MainMenu));
            }
            Event::KeyPressed { key: Key::Z } => {
                if input::is_key_modifier_down(ctx, KeyModifier::Shift) {
                    self.ship_view().dec_zoom(ctx);
                } else {
                    self.ship_view().inc_zoom(ctx);
                }
            }
            Event::MouseWheelMoved { amount } => {
                if amount.y > 0 {
                    self.ship_view().inc_zoom(ctx);
                } else {
                    self.ship_view().dec_zoom(ctx);
                }
            }
            Event::KeyPressed { key: Key::O } => {
                // self.game_mode = GameMode::Activating(Some(Activation::Open), None);
            }
            Event::KeyPressed { key: Key::C } => {
                // self.game_mode = GameMode::Activating(Some(Activation::Close), None);
            }
            Event::KeyPressed { key: Key::A } => {
                // self.game_mode = GameMode::Activating(None, None);
            }
            _ => {}
        }

        None
    }

    fn ui_sprites(&self) -> Option<&[Box<dyn UiSprite>]> {
        Some(&self.sprites)
    }

    fn ui_sprites_mut(&mut self) -> Option<&mut [Box<dyn UiSprite>]> {
        Some(&mut self.sprites)
    }
}
