use std::{ops::Range, slice::IterMut};

#[derive(Copy, Eq, PartialEq, Clone, Default)]
pub struct ThingID
{
    pub index:u16,
    pub generation:u16
}

#[derive(Clone)]
pub struct Things<T>
{
    things:Box<[(ThingID, Option<T>)]>
}


impl<T> Things<T> where T : Clone + PartialEq + Default
{
    pub fn new() -> Things<T>
    {
        let size = u16::MAX / 4;
        let mut things:Vec<(ThingID, Option<T>)> = vec![(ThingID::default(), None); size as usize];
        for i in 0..things.len()
        {
            things[i].0.index = i as u16;
        }
        Self {
            things:things.into_boxed_slice()
        }
    }

    pub fn get_thing_mut(&mut self, id:ThingID) -> Option<(ThingID, &mut T)>
    {
        let e = &mut self.things[id.index as usize];
        if e.0 == id
        {
            if let Some(thing) = &mut e.1
            {
                return Some((id, thing));
            }
        }

        None
    }

    pub fn iter_mut(&mut self) -> ThingsIntoIterator<T>
    {
        ThingsIntoIterator {
            iter:self.things.iter_mut()
        }
    }

    pub fn delete_thing(&mut self, id:ThingID)
    {
        let i = id.index as usize;
        if self.things[i].0 == id && self.things[i].1 != None
        {
            self.things[i].1 = None;
        }
    }

    pub fn new_thing_replicated(&mut self) -> (ThingID, &mut T)
    {
        let l = self.things.len() / 2;
        let slice = 0..l;
        return self.new_thing_internal(slice);
    }

    pub fn new_thing(&mut self) -> (ThingID, &mut T)
    {
        let l = self.things.len() / 2;
        let slice = l..l*2;
        return self.new_thing_internal(slice);
    }

    fn new_thing_internal(&mut self, slice:Range<usize>) -> (ThingID, &mut T)
    {
        let mut id = ThingID::default();
        let mut success = false;
        for i in slice
        {
            if let None = self.things[i].1
            {
                self.things[i].0.generation += 1; // increment generation
                let mut thing = T::default();
                id = self.things[i].0;
                self.things[i].1 = Some(thing);
                success = true;
                break;
            }
        }
      
        if success
        {
            return self.get_thing_mut(id).unwrap();
        }

        panic!("Was not able to allocate Thing, out of space!");
    }
}

pub struct ThingsIntoIterator<'a, T> where T : Clone
{
    iter:IterMut<'a, (ThingID, Option<T>)>
}

impl<'a, T> Iterator for ThingsIntoIterator<'a, T> where T : Clone
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop
        {
            if let Some(e) = self.iter.next()
            {
                if let Some(thing) = &mut e.1
                {
                    return Some(thing);
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