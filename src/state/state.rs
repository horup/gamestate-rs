use std::io::{Read, Write};
use super::{Things};

#[derive(Clone, Eq, PartialEq)]
pub struct State<T> where T : Copy + Clone + PartialEq + Copy + Default + DeltaSerializable
{
    pub things:Things<T>
}

pub trait DeltaSerializable
{
    fn delta_serialize(current:&Self, previous:&Self, writer:&mut dyn Write);
    fn delta_deserialize(previous:&Self, read:&mut dyn Read) -> Self;
}


impl<T> State<T> where T : Copy + Clone + PartialEq + Copy + Default + DeltaSerializable
{
    pub fn new() -> State<T>
    {
        State {
            things:Things::new()
        }
    }
}

impl<T> DeltaSerializable for State<T> where T : Copy + Clone + PartialEq + Copy + Default + DeltaSerializable
{
    fn delta_serialize(current:&Self, previous:&Self, writer:&mut dyn Write) 
    {
        Things::delta_serialize(&current.things, &previous.things, writer);
    }

    fn delta_deserialize(previous:&Self, read:&mut dyn Read) -> Self {
        State {
            things:Things::delta_deserialize(&previous.things, read)
        }
    }
}