use specs;
use ggez_goodies::asset;

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
