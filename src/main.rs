mod events;
mod components;
mod app;
mod state;

use std::{error::Error, io::{stdout}};

use termion::raw::{IntoRawMode};
use tui::backend::TermionBackend;

use crate::app::App;


fn main() -> Result<(), Box<dyn Error>>{

    let stdout = stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);

    let mut app = App::new(backend)?;
    app.run()?;
    Ok(())
}
