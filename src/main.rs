use g::*;
use termion::{clear, cursor, event, input::TermRead, style};
use std::{error::Error, io::stdin};


fn main() -> Result<(), Box<dyn Error>>{
    // In case the program panics, this runs to restore terminal to normal state, otherwise colors won't be reset
    // Todo: figure out how to make it also make the cursor visible
    std::panic::set_hook(Box::new(|err| {
        print!("{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1));
        println!("\r\n{err}");
        }));
    let player = Player::new();
    let mut state = State::new(player, Position::new(1, 1), "tests".to_string());
    
    state.map.replace_tile_at(10, 10, map::Tile{
        material: Material::new('?', (0, 255, 255)),
        can_walk_on: false,
        object: None
    }).unwrap();
    state.display_map();
    
    let stdin: std::io::Stdin = stdin();
    for k in stdin.keys() {
        match k.as_ref().unwrap() {
            event::Key::Char('q') => break,
            event::Key::Char('r') => state.display_map(),
            event::Key::Char('p') => print!("{:?}", state.get_player_pos()),
            event::Key::Char('s') => state.save_map(),
            event::Key::Left => {
                state.move_player(Movement::new(-1, 0));
            },
            event::Key::Right => {
                state.move_player(Movement::new(1, 0));
            },
            event::Key::Up => {
                state.move_player(Movement::new(0, -1));
            },
            event::Key::Down => {
                state.move_player(Movement::new(0, 1));
            },
            _ => ()
        }
        state.flush_stdout();
    }

    print!("{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1));
    Ok(())
}
