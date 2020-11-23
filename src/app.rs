use std::{collections::HashMap, error::Error, io::{self}};

use tui::{Terminal, backend::{Backend}};

use crate::{components::{chat::Chat, Component, login::Login}, events::EventHandler, state::State};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Windows {
    Login,
    Chat
}

pub struct App<B: Backend> {
    event_handler: EventHandler,
    windows: HashMap::<Windows, Box<dyn Component<B>>>,
    state: State,
    terminal: Terminal<B>
}

impl<B: Backend> App<B> {
    pub fn new(backend: B) -> io::Result<Self> {

        let terminal = Terminal::new(backend)?;

        let event_handler = EventHandler::new();

        let mut windows = HashMap::<Windows, Box<dyn Component<B>>>::new();

        let login = Login::new();
        let chat = Chat::new();

        windows.insert(Windows::Login, Box::new(login));
        windows.insert(Windows::Chat, Box::new(chat));

        let state = State::new(Windows::Login);

        Ok(App {
            state,
            windows, 
            event_handler: event_handler, 
            terminal 
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>>{

        self.terminal.clear()?;

        let windows = &self.windows;
        let state = &self.state;
        let events = &self.event_handler;

        loop {
            self.terminal.draw(|f| {
                let current = windows.get(state.get_current_window()).unwrap();
                current.render(f);
            })?;

            let current = windows.get(state.get_current_window()).unwrap();
            current.update(&events);
        }

    }
}

