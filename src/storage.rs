use serde;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Storage {
    
}


pub static mut storage: Storage = Storage{};
