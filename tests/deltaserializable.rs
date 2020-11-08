use std::io::Cursor;

use gamestate::*;
mod shared;
use shared::*;

#[test]
fn deltaserializable()
{
    let empty = State::default();
    let mut current = State::default();
    assert_eq!(current, empty);

    let mut buf = Vec::new();
    let n = current.delta_serialize(&empty, &mut buf).unwrap();
    assert_eq!(n, 0);

    let thing = current.entities.new_entity_replicated().unwrap();

    thing.health = 100.0;
    thing.x = 10.0;
    thing.y = 20.0;
    thing.z = 30.0;

    let mut buf = Vec::new();
    let n = current.delta_serialize(&empty, &mut buf).unwrap();
    assert_ne!(n, 0);
    assert_eq!(current.entities.len(), 1);


    let deserialized = State::delta_deserialize(&empty, &mut Cursor::new(&mut buf)).unwrap();
    println!("{:?}", deserialized.entities.get_entity(ID {index:0, generation:1}));

    assert_eq!(deserialized.entities.len(), 1);

    assert!(current == deserialized);
}