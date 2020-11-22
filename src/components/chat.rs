use tui::{backend::Backend, Frame, layout::{Constraint, Direction, Layout}, widgets::{Block, BorderType, Borders}};

use super::Render;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Chat {

}

impl Chat {
    pub fn new() -> Self {
        Chat {}
    }
}

impl<B: Backend> Render<B> for Chat {
    fn render(&self, f: &mut Frame<B>) {
        
        let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Chat")
                .border_type(BorderType::Rounded);
            f.render_widget(block, size);
    }
}