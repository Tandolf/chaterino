use std::process;

use termion::event::{Key};
use tui::{backend::Backend, Frame, widgets::{Block, BorderType, Borders}};

use crate::events::{Event, EventHandler};

use super::{Component};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Chat {

}

impl Chat {
    pub fn new() -> Self {
        Chat {}
    }
}

impl<B: Backend> Component<B> for Chat {
    fn render(&self, f: &mut Frame<B>) {
        
        let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Chat")
                .border_type(BorderType::Rounded);
            f.render_widget(block, size);
    }

    fn update(&self, events: &EventHandler) {

        if let Event::Input(k) = events.next().unwrap() {
            if k == Key::Char('q') {
                process::exit(0);
            }
        }

    }
}

