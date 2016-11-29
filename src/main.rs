extern crate ggez;
extern crate ggez_goodies;
extern crate specs;
extern crate nalgebra;
use ggez::conf;
use ggez::game::{Game, GameState};
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::timer;
use ggez_goodies::asset;
use ggez_goodies::asset::StateLoadable;

use specs::Join;
use nalgebra as na;

use std::rc::Rc;
use std::time::Duration;

type Vec2 = na::Vector2<f64>;

#[derive(Clone, Debug)]
struct CPosition(Vec2);
impl specs::Component for CPosition {
    type Storage = specs::VecStorage<CPosition>;
}

// Just a marker that a particular entity is the player.
#[derive(Clone, Debug, Default)]
struct CPlayer;

impl specs::Component for CPlayer {
    type Storage = specs::NullStorage<CPlayer>;
}


#[derive(Clone, Debug)]
struct CImage(asset::AssetHandle);
impl specs::Component for CImage {
    type Storage = specs::VecStorage<CImage>;
}

// #[derive(Clone, Debug)]
// struct CImage(graphics::Image);
// impl specs::Component for CImage {
// type Storage = specs::VecStorage<CImage>;
// }
//

struct Assets<'a> {
    images: asset::AssetCache2<&'a str, graphics::Image>,
}

impl<'a> Assets<'a> {
    fn new() -> Self {
        Assets { images: asset::AssetCache2::new() }
    }
}

// First we make a structure to contain the game's state
struct MainState<'a> {
    assets: Assets<'a>,
    // world: specs::World,
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

// Then we implement the `ggez::game::GameState` trait on it, which
// requires callbacks for creating the game state, updating it each
// frame, and drawing it.
//
// The `GameState` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
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

        // For now we just collect all the components into a Vec,
        // and then do our stuff to that vec.
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

// Now our main function, which does three things:
//
// * First, create a new `ggez::conf::Conf`
// object which contains configuration info on things such
// as screen resolution and window title,
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game,
// * then just call `game.run()` which runs the `Game` mainloop.
pub fn main() {
    let c = conf::Conf::new();
    let mut game: Game<MainState> = Game::new("helloworld", c).unwrap();
    if let Err(e) = game.run() {
        println!("Error encountered: {:?}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
