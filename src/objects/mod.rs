use super::*;

pub trait Object{
    fn on_player_contact(&mut self, player: &mut Player);
    fn display(&self) -> Material;
    fn walkable(&self) -> bool;
}

pub struct Objects(pub std::collections::HashMap<Position, Box<dyn Object>>);
impl Objects{
    pub fn new() -> Objects{
        Objects(
            std::collections::HashMap::<Position, Box<dyn Object>>::new()
        )
    }
    pub fn get_obj(&self, position: Position) -> Option<&Box<dyn Object>>{
        self.0.get(&position)
    }
    pub fn get_obj_mut(&mut self, position: Position) -> Option<&mut Box<dyn Object>>{
        self.0.get_mut(&position)
    }
    pub fn add_object<T: Object + 'static>(&mut self, object: T, position: Position){
        self.0.insert(position, Box::from(object));
    }
}
pub struct Switch{
    material_on: Material,
    material_off: Material,
    on_off: bool
}
impl Switch{
    fn change(&mut self){
        self.on_off = !self.on_off;
    }
    fn get_material(&self) -> Material{
        if self.on_off{
            return self.material_on;
        }
        self.material_off
    }
    pub fn new(material_on: Material, material_off: Material) -> Switch{
        Switch{
            material_off,
            material_on,
            on_off: false
        }
    }
}
impl Object for Switch{
    fn on_player_contact(&mut self, _player: &mut Player){
        self.change();
    }
    fn display(&self) -> Material {
        self.get_material()
    }
    fn walkable(&self) -> bool {
        false
    }
}