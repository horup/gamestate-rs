use crate::DeltaSerializable;

#[derive(Copy, Eq, PartialEq, Clone, Default, Debug)]
pub struct ID
{
    pub index:u16,
    pub generation:u16
}

impl ID
{
    pub fn to_be_bytes(&self) -> [u8;4]
    {
        let mut buf= [0;4];
        buf[0..2].copy_from_slice(&self.index.to_be_bytes());
        buf[2..4].copy_from_slice(&self.generation.to_be_bytes());
        self.generation.to_be_bytes();
        return buf;
    }

    pub fn from_be_bytes(bytes:[u8;4]) -> Self
    {
        Self {
            index:u16::from_be_bytes([bytes[0], bytes[1]]),
            generation:u16::from_be_bytes([bytes[2], bytes[3]])
        }
    }

}

pub trait Entity : Copy + Clone + PartialEq + Sized + DeltaSerializable  {
    fn new(id:ID) -> Self;
    fn id(&self) -> ID;
}