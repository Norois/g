use super::{*, objects::*};

pub struct Map{
    pub map: Vec<Vec<Tile>>,
    objects: Objects
 }
impl Map{
    pub fn new(width: u8, height: u8, material: Material) -> Map{
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
        let objects = Objects::new();
        Map{map, objects}
    }
    pub fn get_tile_at(&self, position: Position) -> Option<Tile>{
        self.map.get(position.y as usize)?.get(position.x as usize).copied()
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
    pub fn add_object<T: objects::Object + 'static>(&mut self, object: T, position: Position){
        self.objects.add_object(object, position);
    }
    pub fn get_obj_mut(&mut self, position: Position) -> Option<&mut Box<dyn Object>>{
        self.objects.get_obj_mut(position)
    }
    pub fn get_objects_iter(&self) -> std::collections::hash_map::Iter<'_, Position, Box<dyn objects::Object>>{
        self.objects.0.iter()
    }
 }

#[derive(Clone, Copy, PartialEq)]
pub struct Tile{
    pub material: Material,
    pub can_walk_on: bool
}
impl Tile{
    pub(crate) fn new(material: Material, can_walk_on: bool) -> Tile{
        Tile { material, can_walk_on }
    }
}
