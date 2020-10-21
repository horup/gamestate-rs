use gamestate::*;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Thing
{
    pub x:f32,
    pub y:f32,
    pub health:f32
}

impl DeltaSerializable for Thing
{
    fn delta_serialize(&self, previous:&Self, writer:&mut dyn std::io::Write) -> std::io::Result<usize> {
        Ok(0)
    }

    fn delta_deserialize(previous:&Self, read:&mut dyn std::io::Read) -> std::io::Result<Self> {
        Ok(Self::default())
    }
}

pub type S = State<Thing>;