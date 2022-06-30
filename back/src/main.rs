
mod db;
// use wasm_bindgen::prelude::*;

pub fn main(){
    let db = match db::connect() {
        Ok(database) => database,
        Err(error) => {
            panic!("Cannot connect to instance:: {:?}", error)
        }
    };

println!("{:?}", db);
}