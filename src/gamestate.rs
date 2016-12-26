extern crate ggez;
extern crate ggez_goodies;
extern crate specs;
extern crate nalgebra;
use ggez::conf;
use ggez::event::*;
use ggez::game::GameState;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::timer;
use ggez_goodies::asset;
use ggez_goodies::camera;
use ggez_goodies::input;
// use ggez_goodies::scene;

use specs::Join;

use std::time::Duration;

use components::*;
use util::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Button {
    Fire,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Axis {
    Vert,
    Horz,
}

// /////////////////////////////////////////////////////////////////////
// Global state thingies
// /////////////////////////////////////////////////////////////////////

struct Assets {
    images: asset::AssetCache2<String, graphics::Image>,
}

impl Assets {
    fn new() -> Self {
        Assets { images: asset::AssetCache2::new() }
    }
}

pub struct MainState {
    assets: Assets,
    input: input::InputManager<Axis, Button>,
    planner: specs::Planner<()>,
    camera: camera::Camera,
}

fn create_world() -> specs::World {
    let mut w = specs::World::new();
    w.register::<CPosition>();
    w.register::<CPlayer>();
    w.register::<CImage>();
    w.register::<CBackgroundScroller>();
    w.register::<CCamera>();
    w
}

fn create_player(world: &mut specs::World,
                 assets: &mut Assets,
                 ctx: &mut Context)
                 -> specs::Entity {
    let (handle, _) = assets.images.get_key_state(&"images/kiwi.png".to_string(), ctx).unwrap();
    world.create_now()
        .with(CPosition(Vec2::new(0.0, 0.0)))
        .with(CPlayer)
        .with(CImage(handle))
        .build()
}


fn create_background(world: &mut specs::World,
                     assets: &mut Assets,
                     ctx: &mut Context)
                     -> specs::Entity {
    let (handle, _) =
        assets.images.get_key_state(&"backgrounds/Level1_BG.png".to_string(), ctx).unwrap();
    world.create_now()
        .with(CPosition(Vec2::new(0.0, 0.0)))
        .with(CImage(handle))
        .with(CBackgroundScroller::new())
        .build()
}


fn create_input_manager() -> input::InputManager<Axis, Button> {
    input::InputManager::new()
        .bind_key_to_axis(Keycode::Up, Axis::Vert, true)
        .bind_key_to_axis(Keycode::Down, Axis::Vert, false)
        .bind_key_to_axis(Keycode::Left, Axis::Horz, false)
        .bind_key_to_axis(Keycode::Right, Axis::Horz, true)
        .bind_key_to_button(Keycode::Z, Button::Fire)
}

impl<'a> GameState for MainState {
    fn load(ctx: &mut Context, conf: &conf::Conf) -> GameResult<Self> {

        let mut assets = Assets::new();

        let mut w = create_world();
        let _b = create_background(&mut w, &mut assets, ctx);
        let _p = create_player(&mut w, &mut assets, ctx);
        let c = camera::Camera::new(conf.window_width, conf.window_height, 40.0, 30.0);
        let planner = specs::Planner::new(w, 1);
        let s = MainState {
            assets: assets,
            input: create_input_manager(),
            planner: planner,
            camera: c,
        };
        Ok(s)
    }

    fn update(&mut self, _ctx: &mut Context, dt: Duration) -> GameResult<()> {
        let seconds = timer::duration_to_f64(dt);
        self.input.update(seconds);
        let x_axis = self.input.get_axis(Axis::Horz);
        let y_axis = self.input.get_axis(Axis::Vert);

        if self.input.get_button_down(Button::Fire) {
            println!("Bang!");
        }

        let player_update = move |pos: &mut CPosition, _player: &CPlayer| {
            let xvel = 10.0 * x_axis * seconds;
            let yvel = 10.0 * y_axis * seconds;
            pos.0 += Vec2::new(xvel, yvel);
        };
        let background_update =
            |pos: &mut CPosition, scroller: &CBackgroundScroller| pos.0 += scroller.scroll_speed;
        self.planner.run1w1r(player_update);
        self.planner.run1w1r(background_update);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        ctx.renderer.clear();

        let world = self.planner.mut_world();
        let positions = world.read::<CPosition>();
        let images = world.read::<CImage>();

        for (pos, image) in (&positions, &images).iter() {
            let image = self.assets.images.get_mut(image.0).unwrap();
            let w = image.width();
            let h = image.height();
            let (screen_x, screen_y) = self.camera.world_to_screen_coords(pos.0);
            let r = graphics::Rect::new(screen_x as i32, screen_y as i32, w, h);
            graphics::draw(ctx, image, None, Some(r))?;
        }
        ctx.renderer.present();
        timer::sleep_until_next_frame(ctx, 60);
        Ok(())
    }

    fn key_down_event(&mut self, keycode: Option<Keycode>, _keymod: Mod, _repeat: bool) {
        self.input.update_keydown(keycode);
    }


    fn key_up_event(&mut self, keycode: Option<Keycode>, _keymod: Mod, _repeat: bool) {
        self.input.update_keyup(keycode);
    }
}
