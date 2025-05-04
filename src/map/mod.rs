use super::{*, objects::*};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Map{
    pub map: Vec<Vec<Tile>>,
    pub(crate) size: (u8, u8)
 }
impl Map{
    pub fn new(width: u8, height: u8, material: Material) -> Map{
        let mut map_slice = Vec::new();
        for _ in 0..width{
            map_slice.push(Tile::new(material, true));
        }
        let mut map = Vec::new();
        for _ in 0..height{
            map.push(map_slice.clone());
        }
        Map{map, size: (width, height)}
    }
    pub fn get_tile_at(&self, position: Position) -> Option<&Tile>{
        self.map.get(position.y as usize)?.get(position.x as usize)
    }
    pub fn get_mut_tile_at(&mut self, position: Position) -> Option<&mut Tile>{
        self.map.get_mut(position.y as usize)?.get_mut(position.x as usize)
    }
    pub fn replace_tile_at(&mut self, x: u16, y: u16, tile: Tile) -> Result<(), &str>{
        if let Some(elem) = self
            .map
            .get_mut(y as usize)
            .and_then(|elem| elem.get_mut(x as usize))
        {
            *elem = tile;
        } else{
            return Err("No such tile");
        }
        Ok(())
    }
    pub fn add_object(&mut self, object: Object, position: Position) -> Result<(), &str>{
        if (self.size.1 as u16) < position.x || (self.size.0 as u16) < position.y {
            return Err("Vro this is outside the map lmao");
        }
        self.map.get_mut(position.y as usize).unwrap().get_mut(position.x as usize).unwrap().object = Some(object);
        Ok(())
    }
    
 }

#[derive(Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct Tile{
    pub material: Material,
    pub can_walk_on: bool,
    pub object: Option<Object>
}
impl Tile{
    pub(crate) fn new(material: Material, can_walk_on: bool) -> Tile{
        let object = None;
        Tile { material, can_walk_on, object}
    }
    pub(crate) fn get_material(&self) -> Material{
        if let Some(v) = self.object{v.get_material()}else{self.material}
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub(crate) struct ChunkPosition{
    pub(crate) x: i64,
    pub(crate) y: i64
}
impl ChunkPosition{
    pub(crate) fn new(x: i64, y: i64) -> ChunkPosition{
        ChunkPosition{x, y}
    }
}
