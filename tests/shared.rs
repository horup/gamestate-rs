use std::{io::{Cursor, ErrorKind}};
use std::io::Write;
use std::io::Read;
use gamestate::*;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Thing
{
    pub x:f32,
    pub y:f32,
    pub z:f32,
    pub health:f32
}

fn read_be_f32(read:&mut dyn std::io::Read) -> std::io::Result<f32>
{
    let mut buf = [0 as u8; 4];
    read.read_exact(&mut buf)?;
    return Ok(f32::from_be_bytes(buf));
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
        if self.z != previous.z
        {
            c.write(&[3])?;
            c.write(&self.z.to_be_bytes())?;
        }

        let l = c.position() as usize;

        write.write(&(c.position() as u8).to_be_bytes())?;
        write.write_all(&temp[0..l])?;
        Ok(0)
    }

    fn delta_deserialize(_previous:&Self, read:&mut dyn std::io::Read) -> std::io::Result<Self> {
        let mut current = Self::default();
        let mut buf = [0 as u8; 1024];
        read.read_exact(&mut buf[0..1])?;
        let l = buf[0] as usize;

        read.read_exact(&mut buf[0..l])?;
        let mut cursor = Cursor::new(&buf[0..l]);

        
        while cursor.position() != l as u64 {
            let mut buf = [0 as u8; 1];
            cursor.read_exact(&mut buf[0..1])?;
            match buf[0]
            {
                0 => {
                    current.health = cursor.read_f32::<BigEndian>()?;
                },
                1 => {
                    current.x = cursor.read_f32::<BigEndian>()?;
                },
                2 => {
                    current.y = cursor.read_f32::<BigEndian>()?;
                },
                3 => {
                    current.z = cursor.read_f32::<BigEndian>()?;
                }
                _=> return Err(std::io::Error::new(ErrorKind::Other, "input not understood")),
            }
        }

        Ok(current)
    }
}

pub type S = State<Thing>;