// mod recipes;
mod db;
use wasm_bindgen::prelude::*;

// pub fn main(){
//     let db = match db::connect() {
//                 Ok(database) => database,
//                 Err(error) => {
//                     panic!("Cannot connect to instance:: {:?}", error)
//                 }
//             };
    
//         println!("{:?}", db);
// }

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}