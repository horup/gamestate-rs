use glam::Vec2;


#[derive(Copy, Eq, PartialEq, Clone, Default)]
pub struct ThingID
{
    pub index:u16,
    pub generation:u16
}

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Thing
{
    pub id:ThingID,
    pub pos:Vec2
}