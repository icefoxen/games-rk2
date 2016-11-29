extern crate ggez;
extern crate ggez_goodies;
extern crate specs;
extern crate nalgebra;
use ggez::conf;
use ggez::game::GameState;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::timer;
use ggez_goodies::asset;

use specs::Join;

use std::time::Duration;

use components::*;
use util::*;
/// ///////////////////////////////////////////////////////////////////////
/// Global state thingies
/// ///////////////////////////////////////////////////////////////////////

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
    planner: specs::Planner<()>,
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

impl<'a> GameState for MainState<'a> {
    fn load(ctx: &mut Context, _conf: &conf::Conf) -> GameResult<MainState<'a>> {

        let mut assets = Assets::new();

        let mut w = create_world();
        let _p = create_player(&mut w, &mut assets, ctx);
        let planner = specs::Planner::new(w, 1);
        let s = MainState {
            assets: assets,
            // world: w,
            planner: planner,
        };
        Ok(s)
    }

    fn update(&mut self, _ctx: &mut Context, dt: Duration) -> GameResult<()> {
        let seconds = timer::duration_to_f64(dt);
        let player_update = move |pos: &mut CPosition| {
            println!("Updating player position, is now {:?}, dt is {}",
                     pos,
                     seconds);
            pos.0 += Vec2::new(1.0, 1.0) * seconds;
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

        for (pos, player, image) in (&positions, &playermarkers, &images).iter() {
            println!("Position is: {:?}, {:?}, {:?}", pos, player, image);
            let kiwi = self.assets.images.get_mut(image.0);
            graphics::draw(ctx, kiwi.unwrap(), None, None)?;
        }

        ctx.renderer.present();
        timer::sleep_until_next_frame(ctx, 60);
        Ok(())
    }
}
