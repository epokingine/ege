use crate::{app::App, entity::Entity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Counter {
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct CounterWithName {
    pub super_important_name: String,
    pub count: u32,
}

#[test]
fn add_entites() {
    let mut app = App::new("entity test");

    let counter = Counter { count: -1 };
    let counter_w_name1 = CounterWithName {
        super_important_name: format!("it's really not thta important"),
        count: 0,
    };
    let counter_w_name2 = CounterWithName {
        super_important_name: format!("idk how to stress how unimportant this is"),
        count: 0,
    };

    let entity1 = Entity::create(counter);
    let entity2 = Entity::create(counter_w_name1);
    let entity3 = Entity::create(counter_w_name2);

    let entity2_id = entity2.id();

    assert_ne!(entity1.id(), entity2.id());
    assert_ne!(entity2.id(), entity3.id());

    app.add_entity(entity1);
    app.add_entity(entity2);
    app.add_entity(entity3);

    let try_get_counter_w_name1 = app.get_entity_by_id(entity2_id);
    assert_eq!(try_get_counter_w_name1.is_some(), true);
    let get_all_counters_w_name = app.get_all_entities_by_type::<CounterWithName>();
    assert_eq!(get_all_counters_w_name.len(), 2);
}
