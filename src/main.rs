mod state;
use std::{time::Instant};

use glam::Vec2;
use state::*;

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Sprite
{
    pub texture:u8,
    pub col:u8,
    pub row:u8
}

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Thing
{
    pub id:ThingID,
    pub pos:Vec2,
    pub sprite:Option<Sprite>
}


fn main() {
    let mut now = Instant::now();
    let mut frames = 0;
    let mut state:State<Thing> = State::new();

    loop
    {
        let mut cloned = state.clone();
        
        frames += 1;
        if now.elapsed().as_millis() > 1000
        {
            println!("fps {}", frames);
            frames = 0;
            now = Instant::now();
        }
    }
}
