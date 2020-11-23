use tui::{backend::Backend, Frame};

use crate::events::EventHandler;

pub mod login;
pub mod chat;

pub trait Component<B: Backend> {

    fn render(&self, f: &mut Frame<B>);

    fn update(&self, events: &EventHandler);

}