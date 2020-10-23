use gamestate::*;
mod shared;
use shared::*;

#[test]
fn basics(){
    let empty = S::new();
    let mut current = S::new();
    assert!(empty == current);

    let (id, _) = current.entities.new_entity_replicated().unwrap();
    assert!(empty != current);
    assert_eq!(id, EntityID {index:0, generation:1});

    let (id, t) = current.entities.new_entity_replicated().unwrap();
    t.health = 1.0;
    t.x = 2.0;
    t.y = 3.0;
    assert!(empty != current);
    assert_eq!(id, EntityID {index:1, generation:1});

    current.entities.delete_entity(EntityID {index:0, generation:1});
    let (id, _) = current.entities.new_entity_replicated().unwrap();
    assert_eq!(id, EntityID {index:0, generation:2});
    current.entities.delete_entity(id);

    assert_eq!(current.entities.len(), 1);
    for (_, thing) in current.entities.iter_mut()
    {
        assert_eq!(thing.health, 1.0);
    }

    for _ in 0..10
    {
        current.entities.new_entity_replicated();
    }
    assert_eq!(current.entities.len(), 11);

    current.entities.clear();
    assert_eq!(current.entities.len(), 0);
}