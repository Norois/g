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
    pub position_x: u16,
    pub position_y: u16
}
impl core::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.material)
    }
}

pub struct Map{
   map: Vec<Vec<Tile>>,
   error_tile: Tile,
   draw_at: (u16, u16)
}
impl Map{
    pub fn new(width: u8, height: u8, material: Material, error_tile: Tile, draw_at: (u16, u16)) -> Map{
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
        Map{map, error_tile, draw_at}
    }
    pub fn get_tile_at(&self, x: u16, y: u16) -> Option<Tile>{
        self.map.get(y as usize)?.get(x as usize).copied()
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
        print!("{}", termion::cursor::Goto(self.draw_at.0, self.draw_at.1));
        for slice in &self.map{
            for tile in slice{
                print!("{}", tile.material);
            }
            print!("{}\n", termion::cursor::Left(slice.len() as u16));
        }
        print!("{}{}\n", termion::cursor::Goto(
            player.position_x + self.draw_at.0, 
            player.position_y + self.draw_at.0), player);
    }
    pub fn move_player(&self, player: &mut Player, width: i16, height: i16){
        let new_pos_x = player.position_x.checked_add_signed(width).unwrap_or(player.position_x);
        let new_pos_y = player.position_y.checked_add_signed(height).unwrap_or(player.position_y);

        if let Some(v) = &self.get_tile_at(
            new_pos_x,
            new_pos_y)
        {
            if !v.can_walk_on
            {
                return;
            }
        }
        print!("{}{}", termion::cursor::Goto(player.position_x + self.draw_at.0, player.position_y + self.draw_at.1), self.get_tile_at(player.position_x, player.position_y).unwrap_or(*&self.error_tile).material);
        player.position_x = new_pos_x;
        player.position_y = new_pos_y;
        print!("{}{}\n",termion::cursor::Goto(player.position_x + self.draw_at.0, player.position_y + self.draw_at.1) , player);
    }
}

// trait Object{
//     fn on_player_contact(&self);
// }

// struct Objects(std::collections::HashMap<Position, Box<dyn Object>>);