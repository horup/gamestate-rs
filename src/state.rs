use std::slice::IterMut;

use crate::thing::{Thing, ThingID};

#[derive(Clone)]
pub struct State
{
    pub things:Things
}

impl State
{
    pub fn new() -> State
    {
        State {
            things:Things::new()
        }
    }
}

#[derive(Clone)]
pub struct Things
{
    things:Box<[(u16, Option<Thing>)]>
}


impl Things
{
    pub fn new()->Things
    {
        let size = u16::MAX / 4;
        let things:Vec<(u16, Option<Thing>)> = vec![(0, None); size as usize];
        Self {
            things:things.into_boxed_slice()
        }
    }

    pub fn get_thing_mut(&mut self, id:ThingID) -> Option<&mut Thing>
    {
        let e = &mut self.things[id.index as usize];
        if e.0 == id.generation
        {
            if let Some(thing) = &mut e.1
            {
                return Some(thing);
            }
        }

        None
    }

    pub fn iter_mut(&mut self) -> ThingsIntoIterator
    {
        ThingsIntoIterator {
            iter:self.things.iter_mut()
        }
    }

    pub fn delete_thing(&mut self, id:ThingID)
    {
        let i = id.index as usize;
        if self.things[i].0 == id.generation && self.things[i].1 != None
        {
            self.things[i].1 = None;
        }
    }

    pub fn new_thing_replicated(&mut self) -> &mut Thing
    {
        let l = self.things.len() / 2;
        let mut id = ThingID::default();
        let mut success = false;
        for i in 0..l
        {
            if let None = self.things[i].1
            {
                self.things[i].0 += 1; // increment generation
                let mut thing = Thing::default();
                id = ThingID {
                    generation:self.things[i].0,
                    index:i as u16
                };
                thing.id = id;
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



    pub fn new_thing(&mut self) -> &mut Thing
    {
        let l = self.things.len() / 2;
        let mut id = ThingID::default();
        let mut success = false;
        let t = self.things.iter_mut();

        for i in (l..l*2).rev()
        {
            if let None = self.things[i].1
            {
                self.things[i].0 += 1; // increment generation
                let mut thing = Thing::default();
                id = ThingID {
                    generation:self.things[i].0,
                    index:i as u16
                };
                thing.id = id;
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

pub struct ThingsIntoIterator<'a>
{
    iter:IterMut<'a, (u16, Option<Thing>)>
}

impl<'a> Iterator for ThingsIntoIterator<'a>
{
    type Item = &'a mut Thing;

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
/*
impl <'a> IntoIterator for &'a mut Things
{
    type Item = &'a mut Thing;

    type IntoIter = ThingsIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ThingsIntoIterator {
            iter:self.things.iter_mut()
        }
    }
}*/


/*
pub struct ThingsIntoIterator<'a>
{
    things:&'a mut Things,
    index:usize
}

impl<'a> Iterator for ThingsIntoIterator<'a>
{
    type Item = &'a mut Thing;

    fn next(&mut self) -> Option<Self::Item> {
        let l  = self.things.things.len();
        for i in self.index..l
        {
            self.index = i;

            if let (_, Some(e)) = &mut self.things.things[i]
            {
                return Some(e);
            }
        }

        None
    }
}

impl<'a> IntoIterator for &'a mut Things
{
    type Item = &'a mut Thing;

    type IntoIter = ThingsIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter 
    {
        ThingsIntoIterator {
            things:self,
            index:0
        }
    }
}*/
