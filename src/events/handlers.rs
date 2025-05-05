use super::{
    errors::{EventError, Result},
    handler::EventHandler,
    Event,
};

/// Structure for holding event handlers based on what ``Event`` type it has, this helps with organization.
pub struct EventHandlers {
    pub(crate) setup_handlers: Vec<EventHandler>,
    pub(crate) main_loop_handlers: Vec<EventHandler>,
    pub(crate) timed_handlers: Vec<EventHandler>,
    pub(crate) input_handlers: Vec<EventHandler>,
    pub(crate) conditional_handlers: Vec<EventHandler>,
    pub(crate) shutdown_handlers: Vec<EventHandler>,
}

impl EventHandlers {
    pub fn new() -> Self {
        return EventHandlers {
            setup_handlers: vec![],
            main_loop_handlers: vec![],
            timed_handlers: vec![],
            input_handlers: vec![],
            conditional_handlers: vec![],
            shutdown_handlers: vec![],
        };
    }

    fn check_handlers(
        &self,
        event_type: Event,
        check_handler: &EventHandler,
        handlers: &Vec<EventHandler>,
    ) -> Result<()> {
        if check_handler.event_type != event_type {
            return Err(EventError::WrongEventType);
        }

        for i in 0..handlers.len() {
            if handlers[i].event_type == check_handler.event_type {
                return Err(EventError::IdenticalEventTypes);
            }
        }

        return Ok(());
    }

    pub(crate) fn add_setup_handler(&mut self, handler: EventHandler) -> Result<()> {
        let check = self.check_handlers(Event::Setup, &handler, &self.setup_handlers);

        if check.is_err() {
            return check;
        }

        self.setup_handlers.push(handler);

        return Ok(());
    }

    pub(crate) fn add_main_loop_handler(&mut self, handler: EventHandler) -> Result<()> {
        let check = self.check_handlers(Event::Loop, &handler, &self.setup_handlers);

        if check.is_err() {
            return check;
        }

        self.main_loop_handlers.push(handler);

        return Ok(());
    }

    pub(crate) fn add_timed_handler(&mut self, handler: EventHandler) -> Result<()> {
        let check = self.check_handlers(Event::TimedEvent, &handler, &self.setup_handlers);

        if check.is_err() {
            return check;
        }

        self.timed_handlers.push(handler);

        return Ok(());
    }

    pub(crate) fn add_input_handler(&mut self, handler: EventHandler) -> Result<()> {
        let check = self.check_handlers(Event::Input, &handler, &self.setup_handlers);

        if check.is_err() {
            return check;
        }

        self.input_handlers.push(handler);

        return Ok(());
    }

    pub(crate) fn add_conditional_handler(&mut self, handler: EventHandler) -> Result<()> {
        match handler.event_type {
            Event::Condition(_cond) => {
                let c =
                    self.check_handlers(handler.event_type, &handler, &self.conditional_handlers);
                if c.is_err() {
                    return c;
                }

                self.conditional_handlers.push(handler);

                return Ok(());
            }
            _ => return Err(EventError::WrongEventType),
        }
    }

    pub(crate) fn add_shutdown_handler(&mut self, handler: EventHandler) -> Result<()> {
        let check = self.check_handlers(Event::Shutdown, &handler, &self.setup_handlers);

        if check.is_err() {
            return check;
        }

        self.shutdown_handlers.push(handler);

        return Ok(());
    }

    // Sorts timed events from shortest duration to greatest
    pub(crate) fn sort_timed_events(&mut self) {
        self.timed_handlers.sort();
    }
}
