use std::{io::Read, io::{ErrorKind, Write}, ops::Range, slice::IterMut};
use super::DeltaSerializable;

#[derive(Copy, Eq, PartialEq, Clone, Default, Debug)]
pub struct EntityID
{
    pub index:u16,
    pub generation:u16
}

impl  EntityID
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

#[derive(Clone, PartialEq, Debug)]
pub struct Entities<T>
{
    entities:Box<[(EntityID, Option<T>)]>
}

impl<T> Entities<T> where T : Copy + Clone + PartialEq + Default + DeltaSerializable
{
    pub fn new() -> Entities<T>
    {
        let size = u16::MAX / 4;
        let mut entities:Vec<(EntityID, Option<T>)> = vec![(EntityID::default(), None); size as usize];
        for i in 0..entities.len()
        {
            entities[i].0.index = i as u16;
        }
        Self {
            entities:entities.into_boxed_slice()
        }
    }

    pub fn get_entity_mut(&mut self, id:EntityID) -> Option<(EntityID, &mut T)>
    {
        let e = &mut self.entities[id.index as usize];
        if e.0 == id
        {
            if let Some(e) = &mut e.1
            {
                return Some((id, e));
            }
        }

        None
    }

    pub fn iter_mut(&mut self) -> EntitiesIntoIterator<T>
    {
        EntitiesIntoIterator {
            iter:self.entities.iter_mut()
        }
    }

    pub fn delete_entity(&mut self, id:EntityID)
    {
        let i = id.index as usize;
        if self.entities[i].0 == id && self.entities[i].1 != None
        {
            self.entities[i].1 = None;
        }
    }

    pub fn new_entity_replicated(&mut self) -> Option<(EntityID, &mut T)>
    {
        let l = self.entities.len() / 2;
        let slice = 0..l;
        return self.new_entity_internal(slice);
    }

    pub fn new_entity(&mut self) -> Option<(EntityID, &mut T)>
    {
        let l = self.entities.len() / 2;
        let slice = l..l*2;
        return self.new_entity_internal(slice);
    }

    pub fn clear(&mut self)
    {
        for (_, t) in self.entities.iter_mut()
        {
            *t = None;
        }
    }

    pub fn len(&self) -> usize
    {
        let mut len = 0;
        for (_, e) in self.entities.iter()
        {
            if let Some(_) = e
            {
                len += 1;
            }
        }

        return len;
    }

    fn new_entity_internal(&mut self, slice:Range<usize>) -> Option<(EntityID, &mut T)>
    {
        let mut id = EntityID::default();
        let mut success = false;
        for i in slice
        {
            if let None = self.entities[i].1
            {
                self.entities[i].0.generation += 1; // increment generation
                id = self.entities[i].0;
                self.entities[i].1 = Some(T::default());
                success = true;
                break;
            }
        }
      
        if success
        {
            return self.get_entity_mut(id);
        }

        None
    }
}

impl<T> DeltaSerializable for Entities<T> where T : PartialEq + DeltaSerializable + Copy + Default
{
    fn delta_serialize(&self, previous:&Self, writer:&mut dyn Write) -> std::io::Result<usize> {
        let mut written = 0;
        let l = self.entities.len() / 2; // only first half is replicated
        for i in 0..l
        {
            if self.entities[i] != previous.entities[i] // not equal, thus needs to be delta serialized
            {
                // write id
                written += writer.write(&self.entities[i].0.to_be_bytes())?;
                // write the actual entity data
                match &self.entities[i]
                {
                    (_, None) => written += writer.write(&(0 as u8).to_be_bytes())?, // None entity, write a zero
                    (_, Some(current)) => {
                        written += writer.write(&(1 as u8).to_be_bytes())?; // Some entity, write a one
                        let (_, prev) = &previous.entities[i];
                        let previous = &prev.unwrap_or_default();

                        written += T::delta_serialize(current, previous, writer)?;
                    }
                }
                
            }
        }

        Ok(written)
    }

    fn delta_deserialize(previous:&Self, read:&mut dyn Read) -> std::io::Result<Self> {
        let mut current = previous.clone();
        loop
        {
            // read entityID and then entities.
            // if EOF is found, we have reach the end and no more entities
            let mut buf = [0 as u8;4];
            match read.read_exact(&mut buf)
            {
                Ok(_) => {
                    let id = EntityID::from_be_bytes(buf);
                    let mut buf = [0 as u8;1]; 
                    read.read_exact(&mut buf)?;
                    let has_entity = if buf[0] == 0 { false } else { true };
                    if has_entity
                    {
                        let (_, t) = &previous.entities[id.index as usize];
                        let t = T::delta_deserialize(&t.unwrap_or_default(), read)?;
                        current.entities[id.index as usize] = (id, Some(t));
                    }
                    else
                    {
                        current.entities[id.index as usize] = (id, None);
                    }
                } 
                Err(err) => {
                    let kind = err.kind();
                    if kind == ErrorKind::UnexpectedEof {
                        break;
                    }
                }
            }
            
        }

        Ok(current)
    }
}


pub struct EntitiesIntoIterator<'a, T> where T : Copy + Clone
{
    iter:IterMut<'a, (EntityID, Option<T>)>
}

impl<'a, T> Iterator for EntitiesIntoIterator<'a, T> where T : Copy + Clone
{
    type Item = (EntityID, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        loop
        {
            if let Some(e) = self.iter.next()
            {
                if let Some(thing) = &mut e.1
                {
                    return Some((e.0, thing));
                }
            }
            else
            {
                break;
            }
        }

        None
    }
}