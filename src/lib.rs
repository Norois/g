use std::future::poll_fn;

// https://en.wikipedia.org/wiki/List_of_Unicode_characters#Box_Drawing
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position{
    x: u16,
    y: u16
}
impl Position {
    pub fn new(x: u16, y: u16) -> Position{
        Position{x, y}
    }
    pub fn get_x(&self) -> u16{
        self.x
    }
    pub fn get_y(&self) -> u16{
        self.y
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

#[derive(Clone, Copy, PartialEq)]
pub struct Tile{
    pub material: Material,
    pub can_walk_on: bool
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

pub struct Map{
   map: Vec<Vec<Tile>>,
   error_tile: Tile,
   draw_at: Position,
   objects: objects::Objects
}
impl Map{
    pub fn new(width: u8, height: u8, material: Material, error_tile: Tile, draw_at: Position) -> Map{
        let mut map_slice = Vec::new();
        for _ in 0..width{
            map_slice.push(Tile{
                material,
                can_walk_on: true
            });
        }
        let mut map = Vec::new();
        for _ in 0..height{
            map.push(map_slice.clone());
        }
        let objects = objects::Objects::new();
        Map{map, error_tile, draw_at, objects}
    }
    pub fn get_tile_at(&self, position: Position) -> Option<Tile>{
        self.map.get(position.y as usize)?.get(position.get_x() as usize).copied()
    }
    pub fn replace_tile_at(&mut self, x: u16, y: u16, tile: Tile) -> Result<(), &str>{
        if let Some(elem) = self
            .map
            .get_mut(y as usize)
            .and_then(|elem| elem.get_mut(x as usize))
        {
            *elem = tile;
        } else 
        {
            return Err("No such tile");
        }
        Ok(())
    }
    pub fn display_map(&self, player: &Player){
        print!("{}{}{}\r", termion::clear::All, termion::cursor::Restore, termion::cursor::Hide);
        print!("{}", termion::cursor::Goto(self.draw_at.x, self.draw_at.y));
        for slice in &self.map{
            for tile in slice{
                print!("{}", tile.material);
            }
            print!("{}\n", termion::cursor::Left(slice.len() as u16));
        }
        for object in self.objects.0.iter(){
            print!("{}{}", termion::cursor::Goto(object.0.get_x() + self.draw_at.get_x(), object.0.get_y() + self.draw_at.get_y()), object.1.display())
        }
        print!("{}{}\n", termion::cursor::Goto(
            player.position.x + self.draw_at.x, 
            player.position.y + self.draw_at.y), player);
    }
    pub fn move_player(&mut self, player: &mut Player, width: i16, height: i16){
        let new_position = Position::new(
            player.position.x.checked_add_signed(width).unwrap_or(player.position.x),
            player.position.y.checked_add_signed(height).unwrap_or(player.position.y));

        if let Some(v) = &self.get_tile_at(
            new_position)
        {   
            if self.has_object(new_position){
                if let Some(t) = self.objects.get_obj_mut(new_position){
                    t.on_player_contact();
                    if !t.walkable() {
                        print!("{}{}", termion::cursor::Goto(new_position.get_x() + self.draw_at.get_x(), new_position.get_y() + self.draw_at.get_y()), t.display());
                        return;
                    }
                }
            }
            if !v.can_walk_on
            {
                return;
            }

        } else{
            return;
        }
        print!("{}{}", termion::cursor::Goto(player.position.x + self.draw_at.x, player.position.y + self.draw_at.y), self.get_tile_at(Position::new(player.position.x, player.position.y)).unwrap_or(*&self.error_tile).material);
        player.position.x = new_position.x;
        player.position.y = new_position.y;
        print!("{}{}\n",termion::cursor::Goto(player.position.x + self.draw_at.x, player.position.y + self.draw_at.y) , player);
    }
    pub fn add_object<T: objects::Object + 'static>(&mut self, object: T, position: Position){
        self.objects.0.insert(position, Box::from(object));
    }
    fn has_object(&self, position: Position) -> bool{
        self.objects.0.contains_key(&position)
    }
}

pub mod objects;