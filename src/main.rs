use std::{io::Cursor, io::Write, time::Instant};
use glam::Vec2;
use ruststatetest_rs::*;


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

impl DeltaSerializable for Thing
{
    fn delta_serialize(current:&Self, previous:&Self, writer:&mut dyn Write) {
        todo!()
    }

    fn delta_deserialize(previous:&Self, read:&mut dyn std::io::Read) -> Self {
        let mut res = *previous;
        res
    }
}

fn main() {
    let mut now = Instant::now();
    let mut frames = 0;
    let mut state:State<Thing> = State::new();

    loop
    {
        let mut prev = state.clone();

        frames += 1;
       
        if now.elapsed().as_millis() > 1000
        {
            let mut buffer:Vec<u8> = Vec::new();
            state.things.new_thing_replicated();
            DeltaSerializable::delta_serialize(&state, &prev, &mut buffer);
            let des = DeltaSerializable::delta_deserialize(&state, &mut Cursor::new(&buffer));
            println!("fps {}", frames);
            frames = 0;
            now = Instant::now();
        }
    }
}
