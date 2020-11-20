use std::io::{
    , stdin};
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::{raw::IntoRawMode};
use tui::{
    text::{Span, Spans},
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, List, ListItem, BorderType},
    style::{Color, Modifier, Style}
};

fn get_message<'a>(username: &'a str, message: &'a str, color: Color) -> ListItem<'a> {
    let username = Span::styled(username,Style::default().fg(color));
    let seperator = Span::styled(": ",Style::default().fg(Color::LightMagenta));
    let content = Span::styled(message,Style::default().fg(Color::White));
    let content = vec!(username, seperator, content);
    ListItem::new(Spans::from(content))
}


fn main() -> Result<(), io::Error>{
    let stdin = stdin();
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.

    let rows = vec!(get_message("Toerktumlare", "Hey everyone, whatsapp?", Color::Red), 
        get_message("RobinCheif", "Hey Toerk <3", Color::Cyan),
        get_message("Soffan", "How was your day?", Color::LightBlue),
        get_message("Toerktumlare", "Im fine...", Color::Red), 
        get_message("RobinCheif", "what are you doing Kappa", Color::Cyan),
        get_message("Soffan", "Im painting", Color::LightBlue));
    
    loop {
        terminal.draw(|f| {

            let size = f.size();
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Main block with round corners")
                .border_type(BorderType::Rounded);
            f.render_widget(block, size);
            
            let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints(
                [
                    Constraint::Percentage(100),
                ].as_ref()
            )
            .split(f.size());

            let main = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
            
            f.render_widget(main, chunks[0]);
            f.set_cursor(6, 6);
        })?;
    }
    Ok(())
}
