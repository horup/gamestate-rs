mod state;
mod thing;
use std::{mem, time::Instant};

use state::*;
use thing::*;

fn main() {
    let mut now = Instant::now();
    let mut frames = 0;
    let mut state = State::new();

    for i in 0..10
    {
        let thing = state.things.new_thing_replicated();
        println!("allocated {}", thing.id.index);
    }

    for i in 0..10
    {
        let thing = state.things.new_thing();
        println!("allocated {}", thing.id.index);
    }

    loop
    {
        let mut cloned = state.clone();

        frames += 1;
        if now.elapsed().as_millis() > 1000
        {
            println!("fps {}", frames);
            frames = 0;
            now = Instant::now();
        }
    }
}
