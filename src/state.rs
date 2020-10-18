use std::mem::size_of;

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