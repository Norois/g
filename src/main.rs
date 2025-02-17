use g::*;
use termion::{clear, color::Rgb, cursor, event, input::TermRead, raw::IntoRawMode, style};
use std::{error::Error, io::{stdin, stdout, Write}};


fn main() -> Result<(), Box<dyn Error>>{
    // In case the program panics, this runs to restore terminal to normal state, otherwise colors won't be reset
    std::panic::set_hook(Box::new(|_| {
        print!("{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1));
        println!("The program has panicked!\r");
    }));
    let error_tile = Tile{
        material: Material::new('$', Rgb(255, 0, 255)),
        can_walk_on: false
    };
    let mut map = Map::new(50, 20, Material::new('█', Rgb(0, 255, 0)),
    error_tile, Position::new(1, 1));
    let mut player = Player{
        material: Material::new('ඞ', Rgb(255, 0, 0,)),
        position: Position::new(5, 5)
    };
    
    let stdin: std::io::Stdin = stdin();
    let mut stdout: termion::raw::RawTerminal<std::io::Stdout> = stdout().into_raw_mode()?;
    map.replace_tile_at(10, 10, Tile{
        material: Material::new('?', Rgb(0, 255, 255)),
        can_walk_on: false
    }).unwrap();
    map.add_object(g::objects::Switch::new(Material::new('I', Rgb(0, 255, 0)),
        Material::new('O', Rgb(255, 0, 0))), Position::new(10, 15));
    map.display_map(&player);
    
    for k in stdin.keys() {
        match k.as_ref().unwrap() {
            event::Key::Char('q') => break,
            event::Key::Char('r') => map.display_map(&player),
            event::Key::Char('p') => print!("{:?}", (&player.position.get_x(), &player.position.get_y())),
            event::Key::Left => {
                map.move_player(&mut player, -1, 0);
            },
            event::Key::Right => {
                map.move_player(&mut player, 1, 0);
            },
            event::Key::Up => {
                map.move_player(&mut player, 0, -1);
            },
            event::Key::Down => {
                map.move_player(&mut player, 0, 1);
            },
            _ => ()
        }
        stdout.flush().unwrap();
    }
    stdout.flush().unwrap();

    print!("{}{}{}", clear::All, style::Reset, cursor::Goto(1, 1));
    Ok(())
}
