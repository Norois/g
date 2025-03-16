// https://en.wikipedia.org/wiki/List_of_Unicode_characters#Box_Drawing

pub struct State{
    pub map: Map,
    player: Player,
    draw_at: Position,
    raw_mode: termion::raw::RawTerminal<std::io::Stdout>,
    error_tile: Tile,
    database: data::DataBase
}
impl State{
    pub fn new(map: Map, player: Player, draw_at: Position, path: String) -> State{
        let raw_mode = std::io::stdout().into_raw_mode().unwrap();
        let database = DataBase::new(path);
        let error_tile = Tile::new(Material::new('$', termion::color::Rgb(255, 0, 255)), true);
        State{
            map, 
            player,
            draw_at,
            raw_mode,
            error_tile,
            database
        }
    }
    pub fn display_map(&self){
        print!("{}{}{}\r", termion::clear::All, termion::cursor::Restore, termion::cursor::Hide);
        print!("{}", termion::cursor::Goto(self.draw_at.x, self.draw_at.y));
        for slice in &self.map.map{
            for tile in slice{
                print!("{}", tile.material);
            }
            println!("{}", termion::cursor::Left(slice.len() as u16));
        }
        for object in self.map.get_objects_iter(){
            print!("{}{}", termion::cursor::Goto(object.0.x + self.draw_at.x, object.0.y + self.draw_at.y), object.1.display())
        }
        println!("{}{}", termion::cursor::Goto(
            self.player.position.x + self.draw_at.x, 
            self.player.position.y + self.draw_at.y), self.player);
    }
    pub fn print_player(&self){
        println!("{}{}",termion::cursor::Goto(self.player.position.x + self.draw_at.x, self.player.position.y + self.draw_at.y) , self.player);
    }
    pub fn flush_stdout(&mut self){
        self.raw_mode.flush().unwrap();
    }
    pub fn get_player_pos(&self) -> (u16, u16){
        (self.player.position.x, self.player.position.y)
    }
    pub fn move_player(&mut self, movement: Movement) {
        let new_position = Position::new(
            self.player.position.x.checked_add_signed(movement.width).unwrap_or(self.player.position.x),
            self.player.position.y.checked_add_signed(movement.height).unwrap_or(self.player.position.y));

        if let Some(v) = &self.map.get_tile_at(new_position){   
            if let Some(t) = self.map.get_obj_mut(new_position) {
                t.on_player_contact(&mut self.player);
                if !t.walkable() {
                    print!("{}{}", termion::cursor::Goto(new_position.x + self.draw_at.x, new_position.y + self.draw_at.y), t.display());
                    return;
                }
            }
            if !v.can_walk_on {
                return;
            }

        } else{
            return;
        }
        print!("{}{}", termion::cursor::Goto(self.player.position.x + self.draw_at.x, self.player.position.y + self.draw_at.y), self.map.get_tile_at(Position::new(self.player.position.x, self.player.position.y)).unwrap_or(self.error_tile).material);
        self.player.position.x = new_position.x;
        self.player.position.y = new_position.y;
        self.print_player();
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position{
    x: u16,
    y: u16
}
impl Position {
    pub fn new(x: u16, y: u16) -> Position{
        Position{x, y}
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Material{
    character: char,
    color: termion::color::Rgb
}
impl core::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", termion::color::Fg(self.color), self.character)
    }
}
impl Material{
    pub fn new(character: char, color: termion::color::Rgb) -> Material{
        Material{
            character,
            color
        }
    }
}

pub struct Player{
    pub material: Material,
    pub position: Position
}
impl core::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.material)
    }
}

pub struct Movement{
    width: i16,
    height: i16
}
impl Movement{
    pub fn new(width: i16, height: i16) -> Movement{
        Movement{width, height}
    }
}

use std::io::Write;

use data::DataBase;
pub use map::Map;
use map::Tile;
use termion::raw::IntoRawMode;

pub mod objects;
pub mod map;

pub mod data;


#[cfg(test)]
mod tests{
    use std::{fs::File, io::Write};

    use crate::{data::serde::ToJsonString, Map, Material};

    #[test]
    fn show_me_json(){
        File::create("i").unwrap().write(Map::new(50, 50, Material::new('c', termion::color::Rgb(12, 210, 184))).to_json_string().as_bytes()).unwrap();
    
    }
}