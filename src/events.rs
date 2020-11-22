use termion::{event::Key, input::TermRead};
use std::{time::Duration, io};
use std::sync::mpsc;
use std::thread;

pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration
}

impl Default for Config {
    fn default() -> Config {
        Config { 
            exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(250)
        }
    }
}

pub struct EventHandler {
    pub receiver: mpsc::Receiver<char>
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
                for keyEvent in stdin.keys() {

                    match keyEvent.unwrap() {
                        Key::Char(c) => sender.send(c).unwrap(),
                        _ => break,
                    }
                }
            })
        };

        EventHandler { receiver }
    }
}