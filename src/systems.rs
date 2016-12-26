
use specs;
use specs::Join;

use components::*;
// use ggez_goodies::input;
// use input::*;

#[derive(Debug, Default)]
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

// A System for managing newtonian movement;
// runs on a fixed timestep though.
// todo: Gravity?
#[derive(Debug, Default)]
pub struct MovementSystem {
    dt: f64,
}
impl MovementSystem {
    pub fn new(dt: f64) -> Self {
        MovementSystem { dt: dt }
    }
}
impl specs::System<()> for MovementSystem {
    fn run(&mut self, arg: specs::RunArg, _context: ()) {
        let (mut positions, mut motions) =
            arg.fetch(|w| (w.write::<CPosition>(), w.write::<CMotion>()));
        for (pos, motion) in (&mut positions, &mut motions).iter() {
            motion.velocity += motion.acceleration * self.dt;
            pos.0 += motion.velocity * self.dt;
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
