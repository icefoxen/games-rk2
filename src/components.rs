use specs;
use ggez_goodies::asset;
// use ggez_goodies::camera;

use util::*;


/// ///////////////////////////////////////////////////////////////////////
/// Components
/// ///////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub struct CPosition(pub Vec2);
impl specs::Component for CPosition {
    type Storage = specs::VecStorage<CPosition>;
}

// Just a marker that a particular entity is the player.
#[derive(Clone, Debug, Default)]
pub struct CPlayer;
impl specs::Component for CPlayer {
    type Storage = specs::NullStorage<CPlayer>;
}


#[derive(Clone, Debug)]
pub struct CImage(pub asset::AssetHandle);
impl specs::Component for CImage {
    type Storage = specs::VecStorage<CImage>;
}

// pub struct CCamera {
//     pub c: camera::Camera,
// }
// impl specs::Component for CCamera {
//     type Storage = specs::HashMapStorage<CCamera>;
// }

// impl CCamera {
//     pub fn new(screen_width: u32, screen_height: u32) -> Self {
//         CCamera { c: camera::Camera::new(screen_width, screen_height, 40.0, 30.0) }
//     }
// }

#[derive(Clone, Debug)]
pub struct CBackgroundScroller {
    pub scroll_speed: Vec2,
}
impl specs::Component for CBackgroundScroller {
    type Storage = specs::HashMapStorage<CBackgroundScroller>;
}

impl CBackgroundScroller {
    pub fn new() -> Self {
        CBackgroundScroller { scroll_speed: Vec2::new(0.0, -0.01) }
    }
}
