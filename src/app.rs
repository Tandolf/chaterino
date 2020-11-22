use std::{collections::HashMap, error::Error, io::{self, stdout}};

use termion::raw::IntoRawMode;
use tui::{Terminal, backend::{Backend, TermionBackend}};

use crate::{components::{chat::Chat, Render, login::Login}, events::EventHandler};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Windows {
    Login,
    Chat
}

pub struct App<B: Backend> {
    event_handler: EventHandler,
    windows: HashMap::<Windows, Box<dyn Render<B>>>,
    current_window: Windows,
    terminal: Terminal<B>
}

impl<B: Backend> App<B> {
    pub fn new(backend: B) -> io::Result<Self> {

        let terminal = Terminal::new(backend)?;

        let event_handler = EventHandler::new();

        let mut windows = HashMap::<Windows, Box<dyn Render<B>>>::new();

        let login = Login::new();
        let chat = Chat::new();

        windows.insert(Windows::Login, Box::new(login));
        windows.insert(Windows::Chat, Box::new(chat));

        Ok(App { 
            current_window: Windows::Login, 
            windows, 
            event_handler: event_handler, 
            terminal 
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>>{

        self.terminal.clear()?;

        let windows = &self.windows;
        let current = &self.current_window;

        loop {
            self.terminal.draw(|f| {
                windows.get(current).unwrap().render(f);
            })?
        }
    }
}

