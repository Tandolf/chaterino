use std::{process};

use termion::event::Key;
use tui::{backend::Backend, Frame, layout::{Constraint, Direction, Layout}, widgets::{Block, BorderType, Borders}};

use crate::events::{Event, EventHandler};

use super::{Component};
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Login {

}

impl Login {
    pub fn new() -> Self {
        Login {}
    }
}

impl<B: Backend> Component<B> for Login {
    fn render(&self, f: &mut Frame<B>) {
        let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);
            f.render_widget(block, size);
            

            let vert_margin = (size.height / 2) - 2;
            let chunks = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(5)
            .vertical_margin(vert_margin)
            .constraints(
                [
                    Constraint::Percentage(100),
                ].as_ref()
            )
            .split(f.size());

            let main = Block::default()
            .borders(Borders::ALL)
            .title("Login")
            .border_type(BorderType::Rounded);
            
            f.render_widget(main, chunks[0]);
    }

    fn update(&self, events: &EventHandler) {
        if let Event::Input(k) = events.next().unwrap() {
            if k == Key::Char('q') {
                process::exit(0);
            }
        }
    }
}