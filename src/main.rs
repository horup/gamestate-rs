mod state;
mod thing;
use std::{time::Instant};

use state::*;
use thing::*;

fn main() {
    let mut now = Instant::now();
    let mut frames = 0;
    let mut state = State::new();

    for i in 0..10
    {
        let thing = state.things.new_thing_replicated();
    }

    for i in 0..10
    {
        let thing = state.things.new_thing();
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
