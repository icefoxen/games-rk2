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
use ggez_goodies::input;

use specs::Join;

use std::time::Duration;

use components::*;
use util::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Button {
    Nuffin,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Axis {
    Vert,
    Horz,
}

// /////////////////////////////////////////////////////////////////////
// Global state thingies
// /////////////////////////////////////////////////////////////////////

struct Assets<'a> {
    images: asset::AssetCache2<&'a str, graphics::Image>,
}

impl<'a> Assets<'a> {
    fn new() -> Self {
        Assets { images: asset::AssetCache2::new() }
    }
}

pub struct MainState<'a> {
    assets: Assets<'a>,
    input: input::InputManager<Axis, Button>,
    planner: specs::Planner<()>,
    screen_dimensions: (u32, u32),
}

fn create_world() -> specs::World {
    let mut w = specs::World::new();
    w.register::<CPosition>();
    w.register::<CPlayer>();
    w.register::<CImage>();
    w
}

fn create_player(world: &mut specs::World,
                 assets: &mut Assets,
                 ctx: &mut Context)
                 -> specs::Entity {
    let (handle, _) = assets.images.get_key_state(&"images/kiwi.png", ctx).unwrap();
    world.create_now()
        .with(CPosition(Vec2::new(0.0, 0.0)))
        .with(CPlayer)
        .with(CImage(handle))
        .build()
}

fn create_input_manager() -> input::InputManager<Axis, Button> {
    input::InputManager::new()
        .bind_key_to_axis(Keycode::Up, Axis::Vert, true)
        .bind_key_to_axis(Keycode::Down, Axis::Vert, false)
        .bind_key_to_axis(Keycode::Left, Axis::Horz, false)
        .bind_key_to_axis(Keycode::Right, Axis::Horz, true)
}

/// Translate a world-space coordinate with the origin at the center
/// to a screen-space coordinate with the origin at the upper-left.
/// 'cause nobody uses CRT's anymore.
fn world_to_screen_coords(location: Vec2, screen_dims: (u32, u32)) -> (u32, u32) {
    let (sw, sh) = screen_dims;
    let width = sw as f64;
    let height = sh as f64;
    let x = location.x + width / 2.0;
    let y = height - (location.y + height / 2.0);
    (x as u32, y as u32)
}

impl<'a> GameState for MainState<'a> {
    fn load(ctx: &mut Context, conf: &conf::Conf) -> GameResult<MainState<'a>> {

        let mut assets = Assets::new();

        let mut w = create_world();
        let _p = create_player(&mut w, &mut assets, ctx);
        let planner = specs::Planner::new(w, 1);
        let s = MainState {
            assets: assets,
            input: create_input_manager(),
            planner: planner,
            screen_dimensions: (conf.window_width, conf.window_height),
        };
        Ok(s)
    }

    fn update(&mut self, _ctx: &mut Context, dt: Duration) -> GameResult<()> {
        let seconds = timer::duration_to_f64(dt);
        self.input.update(seconds);
        let x_axis = self.input.get_axis(Axis::Horz);
        let y_axis = self.input.get_axis(Axis::Vert);

        let player_update = move |pos: &mut CPosition| {
            let xvel = 100.0 * x_axis * seconds;
            let yvel = 100.0 * y_axis * seconds;
            pos.0 += Vec2::new(xvel, yvel);
        };
        self.planner.run1w0r(player_update);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        ctx.renderer.clear();

        let world = self.planner.mut_world();
        let positions = world.read::<CPosition>();
        let playermarkers = world.read::<CPlayer>();
        let images = world.read::<CImage>();

        for (pos, _player, image) in (&positions, &playermarkers, &images).iter() {
            // println!("Position is: {:?}, {:?}, {:?}", pos, player, image);
            let kiwi = self.assets.images.get_mut(image.0).unwrap();
            let w = kiwi.width();
            let h = kiwi.height();
            let (screen_x, screen_y) = world_to_screen_coords(pos.0, self.screen_dimensions);
            let r = graphics::Rect::new(screen_x as i32, screen_y as i32, w, h);
            graphics::draw(ctx, kiwi, None, Some(r))?;
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
