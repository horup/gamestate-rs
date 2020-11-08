use std::{slice::Iter, io::Read, io::{ErrorKind, Write}, ops::Range, slice::IterMut};
use crate::{Entity, ID};
use super::DeltaSerializable;

#[derive(Clone, PartialEq, Debug)]
pub struct Collection<T> where T : Entity
{
    entities:Box<[InUse<T>]>
}

impl<T> Default for Collection<T> where T : Entity
{
    fn default() -> Self {
        let size = u16::MAX / 4;
        let mut entities = Vec::new();
        for i in 0..size {
            entities.push(InUse::False(T::new(ID { index:i, generation:0 })));
        }
        Self {
            entities:entities.into_boxed_slice()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum InUse<T>
{
    False(T),
    True(T)
}

impl<T> Collection<T> where T : Entity
{
    pub fn get_entity_mut(&mut self, id:ID) -> Option<&mut T>
    {
        if let InUse::True(e) = &mut self.entities[id.index as usize]
        {
            if e.id() == id
            {
                return Some(e);
            }
        }

        None
    }

    pub fn get_entity(&self, id:ID) -> Option<&T>
    {
        if let InUse::True(e) = &self.entities[id.index as usize]
        {
            if e.id() == id
            {
                return Some(&e);
            }
        }

        None
    }

    pub fn iter_mut(&mut self) -> EntitiesIntoIteratorMut<T>
    {
        EntitiesIntoIteratorMut {
            iter:self.entities.iter_mut()
        }
    }

    pub fn iter(&self) -> EntitiesIntoIterator<T>
    {
        EntitiesIntoIterator {
            iter:self.entities.iter()
        }
    }

    pub fn delete_entity(&mut self, id:ID)
    {
        let i = id.index as usize;
        if let InUse::True(e) = &self.entities[i] {
            if e.id() == id {
                self.entities[i] = InUse::False(*e);
            }
        }
    }

    pub fn new_entity_replicated(&mut self) -> Option<&mut T>
    {
        let l = self.entities.len() / 2;
        let slice = 0..l;
        return self.new_entity_internal(slice);
    }

    pub fn new_entity(&mut self) -> Option<&mut T>
    {
        let l = self.entities.len() / 2;
        let slice = l..l*2;
        return self.new_entity_internal(slice);
    }

    pub fn clear(&mut self)
    {
        for e in self.entities.iter_mut() {
            if let InUse::True(ee) = e {
                *e = InUse::False(*ee);
            }
        }
    }

    pub fn len(&self) -> usize
    {
        let mut len = 0;
        for e in self.entities.iter() {
            if let InUse::True(_) = e {
                len += 1;
            }
        }

        return len;
    }

    fn new_entity_internal(&mut self, slice:Range<usize>) -> Option<&mut T>
    {
        let mut id = ID::default();
        let mut success = false;
        for i in slice
        {
            if let InUse::False(e) = self.entities[i] {
                id = e.id();
                id.generation += 1;
                self.entities[i] = InUse::True(T::new(id));
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

impl<T> DeltaSerializable for Collection<T> where T : Entity
{
    fn delta_serialize(&self, previous:&Self, writer:&mut dyn Write) -> std::io::Result<usize> {
        let mut written = 0;
        let l = self.entities.len() / 2; // only first half is replicated
     /*   for i in 0..l
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
        }*/

        Ok(written)
    }

    fn delta_deserialize(previous:&Self, read:&mut dyn Read) -> std::io::Result<Self> {
        let mut current = previous.clone();
        loop
        {
            // read entityID and then entities.
            // if EOF is found, we have reach the end and no more entities
            let mut buf = [0 as u8;4];
          /*  match read.read_exact(&mut buf)
            {
                Ok(_) => {
                    let id = ID::from_be_bytes(buf);
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
            }*/
            
        }

        Ok(current)
    }
}


pub struct EntitiesIntoIteratorMut<'a, T> where T : Entity
{
    iter:IterMut<'a, InUse<T>>
}

impl<'a, T> Iterator for EntitiesIntoIteratorMut<'a, T> where T : Entity
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop
        {
            if let Some(e) = self.iter.next()
            {
                if let InUse::True(e) = e {
                    return Some(e);
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

pub struct EntitiesIntoIterator<'a, T> where T : Entity
{
    iter:Iter<'a, InUse<T>>
}


impl<'a, T> Iterator for EntitiesIntoIterator<'a, T> where T : Entity
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop
        {
            if let Some(e) = self.iter.next()
            {
                if let InUse::True(e) = e {
                    return Some(e);
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