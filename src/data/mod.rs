use core::panic;
use std::{fs, io::ErrorKind};

use crate::{ChunkPosition, Map};

pub struct DataBase{
    save_folder: String
}
impl DataBase{
    pub fn new(folder: String) -> DataBase{
        let dir = std::fs::DirBuilder::new();
        if !std::fs::exists("savedata").unwrap(){
            match dir.create("savedata"){
                Ok(_) => (),
                Err(e) => match e.kind(){
                    ErrorKind::AlreadyExists => (),
                    ErrorKind::PermissionDenied => panic!("No permission to create a savefile."
                    /* Also to be changed once menu system exists */),
                    _ => panic!{"Bro how did this fail, good luck my guy"}
                }
            }
        }
        match dir.create(format!("savedata/{}", &folder)){
            Ok(()) => DataBase{save_folder: folder},
            Err(v) => match v.kind() {
                ErrorKind::AlreadyExists => DataBase {save_folder: folder},
                ErrorKind::PermissionDenied => panic!("No permission to create a savefile."),
                _ => panic!{"Bro how did this fail, good luck my guy"}
            }
        }
    }
    pub(crate) fn load_map(&self, pos: &ChunkPosition) -> Result<Map, String>{
        let file = match fs::File::open(format!("savedata/{}/{}_{}/map.json", self.save_folder, pos.x, pos.y)){
            Ok(v) => v,
            Err(v) => return Err(v.to_string())
        };
        match serde_json::from_reader(std::io::BufReader::new(file)){
            Ok(v) => Ok(v),
            Err(v) => return Err(v.to_string())
        }
    }
    pub(crate) fn save_map(&self, map: &Map, pos: &ChunkPosition){
        if !std::fs::exists(format!("savedata/{}/{}_{}", self.save_folder, pos.x, pos.y)).unwrap(){
            std::fs::create_dir(format!("savedata/{}/{}_{}", self.save_folder, pos.x, pos.y)).unwrap();
        }
        let file = match fs::File::create(format!("savedata/{}/{}_{}/map.json", self.save_folder, pos.x, pos.y)){
            Ok(v) => v,
            Err(e) => match e.kind(){
                ErrorKind::AlreadyExists => match fs::File::open(format!("savedata/{}/{}_{}/map.json", self.save_folder, pos.x, pos.y)){
                    Ok(v) => v,
                    Err(v) => panic!("Future me please change that when you figure out menus, {v:?}")
                }
                _ => panic!("Future me please change that when you figure out menus {e:?}")
            }
        };
        serde_json::to_writer(file, &map).unwrap();
    }
    pub(crate) fn request_map(&self, chunk: &ChunkPosition, width: u8, height: u8) -> Map{
        if std::fs::exists(format!("savedata/{}/{}_{}", self.save_folder, chunk.x, chunk.y)).unwrap(){
            self.load_map(&chunk).unwrap()
        } else {
            /* Replace later with worldgen stuff, rand color for now hihi */
            
            Map::new(width, height, crate::Material { character: 'K', color: (rand::random_range(0..255),rand::random_range(0..255),rand::random_range(0..255)) })
        }
    }

}