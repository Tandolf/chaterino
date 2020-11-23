use crate::app::Windows;

pub struct State {
    current_window: Windows,
}

impl State {

    pub fn new(current_window: Windows) -> Self {
        State { current_window }
    }

    pub fn get_current_window(&self) -> &Windows {
        &self.current_window
    }
}