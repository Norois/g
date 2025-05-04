// https://en.wikipedia.org/wiki/List_of_Unicode_characters#Box_Drawing
// To any reviewers, I put this link here for future use
pub struct State{
    pub map: Map,
    player: Player,
    draw_at: Position,
    error_tile: Tile,
    database: data::DataBase
}
impl State{
    pub fn new(draw_at: Position, path: String) -> State{
        let database = DataBase::new(path);
        let error_tile = Tile::new(Material::new('$', (255, 0, 255)), true);
        /* Error tile appears only when I break something really bad lol */
        let player = database.request_player();
        let map = database.request_map(&player.chunk_pos, 50, 25);
        
        crossterm::terminal::enable_raw_mode().unwrap();
        State{
            map, 
            player,
            draw_at,
            error_tile,
            database
        }
    }
    pub fn display_map(&self){
        let mut stdout = stdout();
        stdout.execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
        stdout.execute(crossterm::cursor::Hide).unwrap();
        stdout.execute(crossterm::cursor::MoveTo(self.draw_at.x, self.draw_at.y)).unwrap();

        let terminal_size = crossterm::terminal::size().unwrap();
        if terminal_size.0 < self.map.size.0 as u16 || terminal_size.1 < self.map.size.1 as u16 {
            println!("The size of your map is: {:?}.\n\rThe size of your terminal is: {:?}.\n\rPlease Decrease the font size or increase the terminal window to properly display the map and click the R key to reload the map.", self.map.size, terminal_size);
            return;
        }
        

        for slice in &self.map.map{
            for tile in slice{
                if let Some(v) = tile.object{print!("{}", v.get_material())}else{print!("{}", tile.material);}
            }
            print!("\n");
            stdout.execute(crossterm::cursor::MoveLeft(slice.len() as u16)).unwrap();
        }
        stdout.execute(crossterm::cursor::MoveTo(
            self.player.position.x + self.draw_at.x,
            self.player.position.y + self.draw_at.y)
        ).unwrap();
        print!("{}\n", self.player);
    }
    pub fn print_player(&self){
        stdout().execute(crossterm::cursor::MoveTo(self.player.position.x + self.draw_at.x, self.player.position.y + self.draw_at.y)).unwrap();
        println!("{}", self.player);
    }
    pub fn flush_stdout(&mut self){
        stdout().flush().unwrap();
    }
    pub fn get_player_pos(&self) -> (u16, u16){
        (self.player.position.x, self.player.position.y)
    }
    pub fn move_player(&mut self, movement: Movement) {
        let new_position = Position::new(
            self.player.position.x.checked_add_signed(movement.width).unwrap_or(self.player.position.x),
            self.player.position.y.checked_add_signed(movement.height).unwrap_or(self.player.position.y)
        );
        let true_new_position = IPosition::new(
            self.player.position.x as i16 + movement.width,
            self.player.position.y as i16 + movement.height
        );
        if let None = self.map.get_tile_at(true_new_position.to_pos()){
            
            self.save();
            if true_new_position.x >= self.map.size.0 as i16{
                self.player.chunk_pos.x += 1;
                self.player.position.x = 0;
            }
            if true_new_position.y >= self.map.size.1 as i16{
                self.player.chunk_pos.y += 1;
                self.player.position.y = 0;
            }
            if true_new_position.x == -1{
                self.player.chunk_pos.x -= 1;
                self.player.position.x = self.map.size.0 as u16 - 1;
            }
            if true_new_position.y == -1{
                self.player.chunk_pos.y -= 1;
                self.player.position.y = self.map.size.1 as u16 - 1;
            }
            self.map = self.database.request_map(&self.player.chunk_pos, self.map.size.0, self.map.size.1);
            self.display_map();
            return;
        }
        let tile = self.map.get_mut_tile_at(new_position).unwrap();

        if let Some(o) = &mut tile.object{
            o.on_player_walk();
            if !o.can_walk_on(){
                stdout().execute(crossterm::cursor::MoveTo(new_position.x + self.draw_at.x, new_position.y + self.draw_at.y)).unwrap();
                print!("{}", o.get_material());
                return;
            }
        }
        if !tile.can_walk_on{
            return;
        }
        stdout().execute(crossterm::cursor::MoveTo(self.player.position.x + self.draw_at.x, self.player.position.y + self.draw_at.y)).unwrap();
        print!("{}", self.map.get_tile_at(Position::new(self.player.position.x, self.player.position.y)).unwrap_or(&self.error_tile).get_material());
        self.player.position = new_position;
        self.print_player();
    }
    pub fn save(&self){
        self.database.save_map(&self.map, &self.player.chunk_pos);
        self.database.save_player(&self.player);
    }
}

pub struct IPosition{
    x: i16,
    y: i16
}
impl IPosition{
    pub(crate) fn new(x: i16, y: i16) -> IPosition{
        IPosition{x, y}
    }
    pub(crate) fn to_pos(&self) -> Position{
        let x = if self.x < 0 {300}else{self.x as u16};
        let y = if self.y < 0 {300}else{self.y as u16};
        Position { x, y}
    }
}

#[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Position{
    x: u16,
    y: u16
}
impl Position {
    pub fn new(x: u16, y: u16) -> Position{
        Position{x, y}
    }
}

#[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Material{
    character: char,
    color: (u8, u8, u8)
}
impl core::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        stdout().execute(crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb { r: self.color.0, g: self.color.1, b: self.color.2 })).unwrap();
        write!(f, "{}", self.character)
    }
}
impl Material{
    pub fn new(character: char, color: (u8, u8, u8)) -> Material{
        Material{
            character,
            color
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Player{
    pub material: Material,
    pub position: Position,
    pub(crate) chunk_pos: ChunkPosition
}
impl Player{
    pub fn new() -> Player{
        Player{
            material: Material::new('ඞ', (255, 0, 0,)),
            position: Position::new(0, 0),
            chunk_pos: ChunkPosition::new(0, 0)
        }
    }
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

pub fn restore_terminal(){
    let mut stdout = stdout();
    stdout.queue(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
    stdout.queue(crossterm::style::ResetColor).unwrap();
    stdout.queue(crossterm::cursor::Show).unwrap();
    stdout.queue(crossterm::cursor::MoveTo(0, 0)).unwrap();
    stdout.flush().unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
}

use std::io::{stdout, Write};

use crossterm::{ExecutableCommand, QueueableCommand};
use data::DataBase;
pub use map::Map;
use map::{ChunkPosition, Tile};

pub mod objects;
pub mod map;
pub mod data;
pub mod menu;