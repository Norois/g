use super::*;

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
pub enum Object{
    Switch(Switch)
}
impl Object{
    pub(crate) fn on_player_walk(&mut self){
        match self{
            Object::Switch(v) => v.change()
        }
    }
    pub(crate) fn get_material(&self) -> Material{
        match self {
            Object::Switch(v) => v.get_material()
        }
    }
    pub(crate) fn can_walk_on(&self) -> bool{
        match self{
            Object::Switch(_) => false
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Copy, Clone)]
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