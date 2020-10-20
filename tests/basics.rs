use ruststatetest_rs::*;

#[derive(Copy, Clone, PartialEq, Default)]
struct Thing
{
    pub x:f32,
    pub y:f32,
    pub health:f32
}

impl DeltaSerializable for Thing
{
    fn delta_serialize(current:&Self, previous:&Self, writer:&mut dyn std::io::Write) {
        todo!()
    }

    fn delta_deserialize(previous:&Self, read:&mut dyn std::io::Read) -> Self {
        todo!()
    }
}

type S = State<Thing>;

#[test]
fn basics(){
    let empty = S::new();
    let mut current = S::new();
    assert!(empty == current);

    let (id, _) = current.things.new_thing_replicated();
    assert!(empty != current);
    assert_eq!(id, ThingID {index:0, generation:1});

    let (id, t) = current.things.new_thing_replicated();
    t.health = 1.0;
    t.x = 2.0;
    t.y = 3.0;
    assert!(empty != current);
    assert_eq!(id, ThingID {index:1, generation:1});

    current.things.delete_thing(ThingID {index:0, generation:1});
    let (id, _) = current.things.new_thing_replicated();
    assert_eq!(id, ThingID {index:0, generation:2});
    current.things.delete_thing(id);

    assert_eq!(current.things.len(), 1);
    for (id, thing) in current.things.iter_mut()
    {
        assert_eq!(thing.health, 1.0);
    }

    for i in 0..10
    {
        current.things.new_thing_replicated();
    }
    assert_eq!(current.things.len(), 11);

    current.things.clear();
    assert_eq!(current.things.len(), 0);
}