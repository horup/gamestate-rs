use std::io::Cursor;
use std::io::Write;
use std::io::Read;
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
    fn delta_serialize(&self, previous:&Self, write:&mut dyn std::io::Write) -> std::io::Result<usize> {
        let mut temp = [0 as u8;1024];
        let mut c = Cursor::new(&mut temp[0..]);
        if self.health != previous.health
        {
            c.write(&[0])?;
            c.write(&self.health.to_be_bytes())?;
        }
        if self.x != previous.x
        {
            c.write(&[1])?;
            c.write(&self.x.to_be_bytes())?;
        }
        if self.y != previous.y
        {
            c.write(&[2])?;
            c.write(&self.y.to_be_bytes())?;
        }

        let l = c.position() as usize;

        write.write(&(c.position() as u8).to_be_bytes())?;
        write.write_all(&temp[0..l])?;
        Ok(0)
    }

    fn delta_deserialize(previous:&Self, read:&mut dyn std::io::Read) -> std::io::Result<Self> {
        let mut buf = [0 as u8; 1];
        read.read_exact(&mut buf)?;
        let l = buf[0] as usize;
        let mut buf = [0 as u8; 1024];
        read.read_exact(&mut buf[0..l])?;
        let mut cursor = Cursor::new(&buf);
        loop {
        }


        Ok(Self::default())
    }
}

pub type S = State<Thing>;