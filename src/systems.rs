
use specs;
use specs::Join;

use components::*;
// use ggez_goodies::input;
// use input::*;

pub struct BackgroundSystem;

impl specs::System<()> for BackgroundSystem {
    fn run(&mut self, arg: specs::RunArg, _context: ()) {
        let (mut positions, scrollers) =
            arg.fetch(|w| (w.write::<CPosition>(), w.read::<CBackgroundScroller>()));
        for (pos, scroller) in (&mut positions, &scrollers).iter() {
            pos.0 += scroller.scroll_speed;
        }
    }
}


// pub struct PlayerMotion;
// impl<'a> specs::System<&'a input::InputManager<Axis, Button>> for BackgroundSystem {
//     fn run(&mut self, arg: specs::RunArg, context: &'a input::InputManager<Axis, Button>) {
//         let (mut positions, scrollers) =
//             arg.fetch(|w| (w.write::<CPosition>(), w.read::<CBackgroundScroller>()));
//         for (pos, scroller) in (&mut positions, &scrollers).iter() {
//             pos.0 += scroller.scroll_speed;
//         }
//     }
// }
