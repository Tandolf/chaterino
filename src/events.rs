use termion::{event::Key, input::TermRead};
use std::{io, time::Duration, process};
use std::sync::mpsc;
use std::thread;

pub struct Config {
    pub exit_key: Key
}

impl Default for Config {
    fn default() -> Config {
        Config { 
            exit_key: Key::Esc
        }
    }
}

pub enum Event<I> {
    Input(I)
}

pub struct EventHandler {
    pub receiver: mpsc::Receiver<Event<Key>>
}

impl EventHandler {

    pub fn new() -> EventHandler {
        EventHandler::from_config(Config::default())
    }

    pub fn from_config(config: Config) -> EventHandler{

        let (sender, receiver) = mpsc::channel();
        let input_handle = {
            let sender = sender.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for key_event in stdin.keys() {
                    match key_event.unwrap() {
                        Key::Char(c) => sender.send(Event::Input(Key::Char(c))).unwrap(),
                        Key::Esc => process::exit(0),
                        _ => break,
                    }
                }
            })
        };

        EventHandler { receiver }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.receiver.recv()
    }
}