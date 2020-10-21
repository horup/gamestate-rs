use std::io::{Read, Write};
use super::{Entities};

#[derive(Clone, PartialEq)]
pub struct State<T> where T : Copy + Clone + PartialEq + Copy + Default + DeltaSerializable
{
    pub entities:Entities<T>
}

pub trait DeltaSerializable
{
    fn delta_serialize(current:&Self, previous:&Self, writer:&mut dyn Write) -> std::io::Result<usize>;
    fn delta_deserialize(previous:&Self, read:&mut dyn Read) -> std::io::Result<Self> where Self : Sized;
}


impl<T> State<T> where T : Copy + Clone + PartialEq + Copy + Default + DeltaSerializable
{
    pub fn new() -> State<T>
    {
        State {
            entities:Entities::new()
        }
    }
}

impl<T> DeltaSerializable for State<T> where T : Copy + Clone + PartialEq + Copy + Default + DeltaSerializable
{
    fn delta_serialize(current:&Self, previous:&Self, writer:&mut dyn Write) -> std::io::Result<usize>
    {
        let n = Entities::delta_serialize(&current.entities, &previous.entities, writer)?;
        Ok(n)
    }

    fn delta_deserialize(previous:&Self, read:&mut dyn Read) -> std::io::Result<Self> {
        Ok(
            State {
            entities:Entities::delta_deserialize(&previous.entities, read)?
        })
    }
}