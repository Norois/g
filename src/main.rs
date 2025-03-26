use crossterm::event::KeyCode;
use g::*;


fn main(){
    // In case the program panics, this runs to restore terminal to normal state, otherwise colors won't be reset
    // Todo: figure out how to make it also make the cursor visible
    std::panic::set_hook(Box::new(|err| {
        g::restore_terminal();
        println!("\r\n{err}");
        }));
    
    let player = Player::new();
    let mut state = State::new(player, Position::new(0, 0), "tests".to_string());
    
    // state.map.replace_tile_at(10, 10, map::Tile{
    //     material: Material::new('?', (0, 255, 255)),
    //     can_walk_on: false,
    //     object: None
    // }).unwrap();
    
    state.display_map();
    
    loop {
        match crossterm::event::read().unwrap() {
            crossterm::event::Event::Key(event) => match event.code{
                KeyCode::Char('q') => break,
                KeyCode::Char('p') => print!("{:?}", state.get_player_pos()),
                KeyCode::Char('s') => state.save_map(),
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
