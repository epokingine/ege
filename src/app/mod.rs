use std::{
    any::Any,
    thread::sleep,
    time::{Duration, SystemTime},
};

use device_query::{DeviceEventsHandler, DeviceState, Keycode, MouseState};
use log::{error, info};
use serde::de::DeserializeOwned;

use crate::{
    entity::Entity,
    events::{
        errors::{EventError, Result},
        handler::EventHandler,
        handlers::EventHandlers,
        Event,
    },
    util::logging::setup_logging,
    window::x11::X11Window,
};

pub struct App {
    title: &'static str,
    delay: Duration,
    event_handlers: EventHandlers,
    enities: Vec<Entity>,
    running: bool,
    time: Option<SystemTime>,
    keys: Vec<Keycode>,
    mouse: MouseState,
    window: Option<X11Window>,
    window_handler: Option<fn(&App, &X11Window)>,
}

impl App {
    pub fn new(title: &'static str) -> Self {
        return App {
            title,
            delay: Duration::new(0, 0),
            event_handlers: EventHandlers::new(),
            enities: vec![],
            running: false,
            time: None,
            keys: vec![],
            mouse: MouseState {
                coords: (0, 0),
                button_pressed: vec![],
            },
            window: None,
            window_handler: None,
        };
    }

    /// This adds a delay that will run at the end of every loop, this will ensure that your app is not running as fast as possible
    /// A delay between 10 and 20 milliseconds is recomended
    pub fn add_delay(&mut self, delay: Duration) {
        self.delay = delay;
    }

    pub fn add_window(&mut self, width: u32, height: u32) {
        let win_result = X11Window::new(width, height, self.title);
        if win_result.is_err() {
            error!("Failed to open window");
            return;
        }

        self.window = Some(win_result.unwrap());
    }

    pub fn add_window_handler(&mut self, handler_fn: fn(&App, &X11Window)) {
        self.window_handler = Some(handler_fn);
    }

    pub fn add_event_handler(&mut self, handler: EventHandler) -> Result<&mut Self> {
        let ret = match handler.event_type {
            Event::Setup => self.event_handlers.add_setup_handler(handler),
            Event::TimedEvent => self.event_handlers.add_timed_handler(handler),
            Event::Loop => self.event_handlers.add_main_loop_handler(handler),
            Event::Condition(_cond) => self.event_handlers.add_conditional_handler(handler),
            Event::Input => self.event_handlers.add_input_handler(handler),
            Event::Shutdown => self.event_handlers.add_shutdown_handler(handler),
        };

        if ret.is_err() {
            return Err(ret.err().unwrap_or(EventError::WrongEventType));
        } else {
            return Ok(self);
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.enities.push(entity);
    }

    pub fn update_entity(&mut self, id: u64, entity: Entity) {
        for i in 0..self.enities.len() {
            println!("searching for entity: {} -- {:?}", id, self.enities[0].id);
            if self.enities[i].id == id {
                println!("found entity");
                self.enities[i] = entity;
                return;
            }
        }
    }

    pub fn get_entity_by_id(&mut self, id: u64) -> Option<&Entity> {
        for i in 0..self.enities.len() {
            if self.enities[i].id == id {
                return Some(&self.enities[i]);
            }
        }

        return None;
    }

    pub fn get_all_entities_by_type<T: Any + DeserializeOwned>(&self) -> Vec<T> {
        let mut ret = vec![];

        for i in 0..self.enities.len() {
            let parse = serde_json::from_value(self.enities[i].contents.clone());
            if parse.is_ok() {
                ret.push(parse.unwrap());
            }
        }

        return ret;
    }

    pub fn elapsed_time(&self) -> Option<Duration> {
        if self.time.is_none() {
            return None;
        } else {
            Some(self.time.unwrap().elapsed().unwrap())
        }
    }

    pub fn current_keys_down(&self) -> Vec<Keycode> {
        return self.keys.clone();
    }

    pub fn currnet_mouse(self) -> MouseState {
        return self.mouse;
    }

    pub fn setup(&mut self) {
        setup_logging();
        info!("Starting {}...", self.title);
        info!("To disable logging use ``logging_disabled`` feature");

        info!("Sorting timed events...");
        self.event_handlers.sort_timed_events();

        info!("Running Setup events...");
        for i in 0..self.event_handlers.setup_handlers.len() {
            (self.event_handlers.setup_handlers[i].handler_fn)(self);
        }

        self.time = Some(SystemTime::now());
        // Delay to prevent timed events from running immediately
        sleep(Duration::new(0, 5000));

        self.running = true;
    }

    fn run_timed_events(&mut self) {
        for i in 0..self.event_handlers.timed_handlers.len() {
            if self.running == false {
                return;
            }

            if self.event_handlers.timed_handlers[i].interval.is_none() {
                continue;
            }

            let event_interval = self.event_handlers.timed_handlers[i].interval.unwrap();
            let modu =
                self.time.unwrap().elapsed().unwrap().as_millis() % event_interval.as_millis();

            if modu == 0 {
                (self.event_handlers.timed_handlers[i].handler_fn)(self);
            }
        }
    }

    fn check_conditional_events(&mut self) {
        for i in 0..self.event_handlers.conditional_handlers.len() {
            match self.event_handlers.conditional_handlers[i].event_type {
                Event::Condition(cond) => {
                    if (cond)(self) {
                        (self.event_handlers.conditional_handlers[i].handler_fn)(self);
                    }
                }
                _ => {}
            }
        }
    }

    fn run_input_events(&mut self) {
        for i in 0..self.event_handlers.input_handlers.len() {
            (self.event_handlers.input_handlers[i].handler_fn)(self)
        }
    }

    pub fn stop(&mut self) {
        self.running = false;

        if self.window.is_some() {
            self.window.as_mut().unwrap().close_window();
        }
    }

    pub fn start(&mut self) {
        self.setup();

        let get_device_events = DeviceEventsHandler::new(Duration::from_millis(5));
        if get_device_events.is_none() {
            error!("Failed to get device input events");
        }

        let device_state = DeviceState::new();

        while self.running {
            self.run_timed_events();
            self.check_conditional_events();

            self.keys = device_state.query_keymap();
            self.mouse = device_state.query_pointer();
            self.run_input_events();

            for i in 0..self.event_handlers.main_loop_handlers.len() {
                (self.event_handlers.main_loop_handlers[i].handler_fn)(self);
            }

            if self.window.is_some() && self.window_handler.is_some() {
                self.window.as_ref().unwrap().clear_window();
                (self.window_handler.unwrap())(&self, self.window.as_ref().unwrap());
            }

            sleep(self.delay);
        }
    }
}
