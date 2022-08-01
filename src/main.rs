mod user_interface;

use std::{
    io, 
    time::Duration
};
use crossterm::{
    event::{poll, read, Event, KeyCode}, 
    terminal::enable_raw_mode
};
use tui::{
    backend::CrosstermBackend, 
    Terminal
};
use user_interface::UserInterface;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    enable_raw_mode()?;

    let mut app_ui = UserInterface::new(vec!["value1".to_string(), "value2".to_string()]);

    loop {

        app_ui.draw(&mut terminal);
        
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(key_event) => {
                    match key_event.code {
                        KeyCode::Backspace => todo!(),
                        KeyCode::Enter => todo!(),
                        KeyCode::Left => todo!(),
                        KeyCode::Right => todo!(),
                        KeyCode::Up => app_ui.list_up(),
                        KeyCode::Down => app_ui.list_down(),
                        KeyCode::Home => todo!(),
                        KeyCode::End => todo!(),
                        KeyCode::PageUp => todo!(),
                        KeyCode::PageDown => todo!(),
                        KeyCode::Tab => todo!(),
                        KeyCode::BackTab => todo!(),
                        KeyCode::Delete => todo!(),
                        KeyCode::Insert => todo!(),
                        KeyCode::F(_) => todo!(),
                        KeyCode::Char(_) => todo!(),
                        KeyCode::Null => todo!(),
                        KeyCode::Esc => todo!(),
                    };
                },
                Event::Mouse(_mouse_event) => {},
                Event::Resize(_width, _height) => {},
           }
        }

    };

    Ok(())
}
            


            


    

