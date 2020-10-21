use std::io::Cursor;

use gamestate::*;
mod shared;
use shared::*;

#[test]
fn deltaserializable()
{
    let empty = S::new();
    let mut current = S::new();
    assert_eq!(current, empty);

    let mut buf = Vec::new();
    let n = current.delta_serialize(&empty, &mut buf).unwrap();
    assert_eq!(n, 0);

    current.entities.new_entity_replicated();
    let mut buf = Vec::new();
    let n = current.delta_serialize(&empty, &mut buf).unwrap();
    assert_ne!(n, 0);
    assert_eq!(current.entities.len(), 1);


    let deserialized = S::delta_deserialize(&empty, &mut Cursor::new(&mut buf)).unwrap();
    assert_eq!(deserialized.entities.len(), 1);
    assert!(current == deserialized);
}