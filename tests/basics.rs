use gamestate::*;
mod shared;
use shared::*;

#[test]
fn basics(){
    let empty = State::default();
    let mut current = State::default();
    assert!(empty == current);

    let t = current.entities.new_entity_replicated().unwrap();
    assert_eq!(t.id, ID {index:0, generation:1});
    assert!(empty != current);

    let t = current.entities.new_entity_replicated().unwrap();
    t.health = 1.0;
    t.x = 2.0;
    t.y = 3.0;
    assert_eq!(t.id, ID {index:1, generation:1});
    assert!(empty != current);


    current.entities.delete_entity(ID {index:0, generation:1});
    let t = current.entities.new_entity_replicated().unwrap();
    assert_eq!(t.id, ID {index:0, generation:2});
    let id = t.id;
    current.entities.delete_entity(id);

    assert_eq!(current.entities.len(), 1);
    for t in current.entities.iter_mut()
    {
        assert_eq!(t.health, 1.0);
    }

    for _ in 0..10
    {
        current.entities.new_entity_replicated();
    }
    assert_eq!(current.entities.len(), 11);

    current.entities.clear();
    assert_eq!(current.entities.len(), 0);
}