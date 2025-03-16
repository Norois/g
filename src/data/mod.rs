pub(crate) struct DataBase{

}
impl DataBase{
    pub(crate) fn new(folder: String) -> DataBase{
        let dir = std::fs::DirBuilder::new();
        match dir.create(folder){
            Ok(()) => DataBase{},
            Err(v) => {println!("{v:?}"); panic!("idk");}
        }
    }
}

pub mod serde;