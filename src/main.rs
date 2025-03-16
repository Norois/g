use g::*;
use termion::{clear, color::Rgb, cursor, event, input::TermRead, style};
use std::{error::Error, io::stdin};


fn main() -> Result<(), Box<dyn Error>>{
    // In case the program panics, this runs to restore terminal to normal state, otherwise colors won't be reset
    std::panic::set_hook(Box::new(|_| {
        print!("{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1));
        println!("The program has panicked!\r");
    }));
    let map = Map::new(75, 30, Material::new('█', Rgb(0, 255, 0)));
    let player = Player{
        material: Material::new('ඞ', Rgb(255, 0, 0,)),
        position: Position::new(0, 0)
    };
    let mut state = State::new(map, player, Position::new(1, 1), "idk".to_string());
    
    // state.map.replace_tile_at(10, 10, Tile{
    //     material: Material::new('?', Rgb(0, 255, 255)),
    //     can_walk_on: false
    // }).unwrap();
    state.map.add_object(g::objects::Switch::new(Material::new('I', Rgb(0, 255, 0)),
        Material::new('O', Rgb(255, 0, 0))), Position::new(10, 15));
    state.map.add_object(g::objects::Switch::new(Material::new('I', Rgb(0, 255, 0)),
    Material::new('O', Rgb(255, 0, 0))), Position::new(10, 15));
    state.display_map();
    
    let stdin: std::io::Stdin = stdin();
    for k in stdin.keys() {
        match k.as_ref().unwrap() {
            event::Key::Char('q') => break,
            event::Key::Char('r') => state.display_map(),
            event::Key::Char('p') => print!("{:?}", state.get_player_pos()),
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
