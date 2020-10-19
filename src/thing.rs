use glam::Vec2;

use crate::state::ThingID;



#[derive(Default, Copy, Clone, PartialEq)]
pub struct Thing
{
    pub id:ThingID,
    pub pos:Vec2
}