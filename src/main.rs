use std::io::{stdout, Write};

use crossterm::QueueableCommand;

use crossterm::event::KeyCode;
use g::*;


fn main(){
    // In case the program panics, this runs to restore terminal to normal state, otherwise colors won't be reset
    // Todo: figure out how to make it also make the cursor visible
    std::panic::set_hook(Box::new(|err| {
        g::restore_terminal();
        println!("\r\n{err}");
        }));

    let mut state = State::new(Position::new(0, 0), "new".to_string());
        
    let mut stdout = stdout();
    stdout.queue(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    stdout.queue(crossterm::style::ResetColor).unwrap();
    stdout.queue(crossterm::cursor::MoveTo(0, 0)).unwrap();
    stdout.queue(crossterm::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    
    let mut menu = g::menu::MainMenu::new();
    menu.process_menu();

    
    state.display_map();
    
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(event) => match event.code{
                KeyCode::Char('q') => break,
                KeyCode::Char('p') => print!("{:?}", state.get_player_pos()),
                KeyCode::Char('s') => state.save(),
                KeyCode::Up => state.move_player(Movement::new(0, -1)),
                KeyCode::Down => state.move_player(Movement::new(0, 1)),
                KeyCode::Left => state.move_player(Movement::new(-1, 0)),
                KeyCode::Right => state.move_player(Movement::new(1, 0)),
                _ => ()
            },
            crossterm::event::Event::Resize(..) => state.display_map(),
            _ => ()
        }
        state.flush_stdout();
    }

    g::restore_terminal();
}
