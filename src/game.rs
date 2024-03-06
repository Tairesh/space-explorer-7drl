use std::cell::RefCell;
use std::rc::Rc;

use tetra::{Context, Event, TetraError};

use crate::assets::Assets;
use crate::data::game_data::GameData;
use crate::scenes::{MainMenu, Scene, SceneEvent, SceneImpl, ShipWalk};
use crate::world::World;

pub struct Game {
    data: Rc<GameData>,
    assets: Rc<Assets>,
    scene: Box<dyn SceneImpl>,
    world: Option<Rc<RefCell<World>>>,
    window_size: (i32, i32),
}

impl Game {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let data = Rc::new(GameData::load());
        let assets = Rc::new(Assets::load(ctx)?);
        let mut game = Self {
            window_size: tetra::window::get_size(ctx),
            scene: Box::new(MainMenu::new(&assets)),
            world: None,
            data,
            assets,
        };
        game.on_open(ctx);
        Ok(game)
    }

    fn set_scene(&mut self, ctx: &mut Context, scene: Scene) {
        match scene {
            Scene::MainMenu => {
                self.scene = Box::new(MainMenu::new(&self.assets));
            }
            Scene::ShipWalk => {
                self.world = Some(Rc::new(RefCell::new(World::new(&self.data))));
                self.scene = Box::new(ShipWalk::new(
                    ctx,
                    self.assets.clone(),
                    self.world.clone().unwrap(),
                ));
            }
        };
        self.on_open(ctx);
    }

    fn on_scene_event(&mut self, ctx: &mut Context, event: SceneEvent) {
        match event {
            SceneEvent::ChangeScene(scene) => self.set_scene(ctx, scene),
            SceneEvent::Custom(e) => {
                if let Some(event) = self.scene.custom_event(ctx, e) {
                    self.on_scene_event(ctx, event);
                }
            }
            SceneEvent::Exit => {
                tetra::window::quit(ctx);
            }
        }
    }

    fn on_open(&mut self, ctx: &mut Context) {
        self.scene.on_open(ctx);
        self.on_resize(ctx, self.window_size);
    }

    fn on_resize(&mut self, ctx: &mut Context, window_size: (i32, i32)) {
        self.window_size = window_size;
        self.scene.reposition_all_sprites(ctx, window_size);
        self.scene.on_resize(ctx, window_size);
    }
}

impl tetra::State for Game {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if let Some(event) = self.scene.on_update(ctx) {
            self.on_scene_event(ctx, event);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        self.scene.before_draw(ctx);
        if let Some(sprites) = self.scene.ui_sprites_mut() {
            for sprite in &mut *sprites {
                sprite.draw(ctx);
            }
        }
        self.scene.after_draw(ctx);

        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> Result<(), TetraError> {
        if let Event::Resized { width, height } = event {
            self.on_resize(ctx, (width, height));
        }

        if let Some(event) = self.scene.on_event(ctx, event) {
            self.on_scene_event(ctx, event);
        }

        Ok(())
    }
}
