use std::io::{Read, Write};
use super::{Collection};

pub trait DeltaSerializable
{
    fn delta_serialize(&self, previous:&Self, write:&mut dyn Write) -> std::io::Result<usize>;
    fn delta_deserialize(previous:&Self, read:&mut dyn Read) -> std::io::Result<Self> where Self : Sized;
}