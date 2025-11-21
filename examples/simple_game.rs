use std::time::Duration;

use ege::{
    app::App,
    entity::Entity,
    events::{handler::EventHandler, Event},
    input::keyboard::Keycode,
    window::{color::RGBColor, x11::X11Window},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
struct Player {
    pub entity_id: u64,
    pub x: u32,
    pub y: u32,
}

fn input_handler(app: &mut App) {
    let keys = app.current_keys_down();
    // There is currently only one player so use index 0
    let mut current_player = app.get_all_entities_by_type::<Player>()[0];

    for i in 0..keys.len() {
        if keys[i] == Keycode::W {
            current_player.y += 1;
        }

        if keys[i] == Keycode::S {
            current_player.y -= 1;
        }

        if keys[i] == Keycode::D {
            current_player.x += 1;
        }

        if keys[i] == Keycode::A {
            current_player.x -= 1;
        }
    }

    app.update_entity(
        current_player.entity_id,
        Entity::recreate(current_player.entity_id, current_player),
    );
}

fn window_handler(app: &App, window: &X11Window) {
    let current_player = app.get_all_entities_by_type::<Player>()[0];

    window.draw_rect(
        RGBColor(0, 255, 0),
        current_player.x,
        current_player.y,
        20,
        20,
    );
}

fn main() {
    let mut app = App::new("Simple Game");
    let mut player = Player {
        entity_id: 0,
        x: 20,
        y: 20,
    };

    let mut player_entity = Entity::create(player);
    let player_entity_id = player_entity.id();
    player.entity_id = player_entity_id;
    player_entity = Entity::recreate(player.entity_id, player);
    app.add_entity(player_entity);

    app.add_event_handler(EventHandler {
        event_type: Event::Input,
        interval: None,
        handler_fn: input_handler,
    })
    .unwrap();

    app.add_window(200, 200);
    app.add_window_handler(window_handler);
    app.add_delay(Duration::from_millis(55));

    app.start();
}
