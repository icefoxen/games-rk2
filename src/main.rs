extern crate ggez;
extern crate ggez_goodies;
extern crate specs;
extern crate nalgebra;
use ggez::conf;
use ggez::game::Game;

pub mod components;
pub mod input;
pub mod systems;
pub mod gamestate;
pub mod util;


use gamestate::*;

pub fn main() {
    let c = conf::Conf::new();
    let mut game: Game<MainState> = Game::new("rk2", c).unwrap();
    if let Err(e) = game.run() {
        println!("Error encountered: {:?}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
