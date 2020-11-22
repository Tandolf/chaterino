use tui::{backend::Backend, Frame};

pub mod login;
pub mod chat;

pub trait Render<B: Backend> {
    fn render(&self, f: &mut Frame<B>);
}