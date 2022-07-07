
mod db;

// use wasm_bindgen::prelude::*;
use mongodb::{bson, Database, Collection};
use chrono::{TimeZone, Utc};

pub fn main(){
    let rice = db::model::recipes_model::Recipes {
        _id: None,
        name: "Rice".to_owned(),
        ingredients: vec![db::model::recipes_model::Ingredient {
            _id: None,
            name: "rice".to_owned(),
            quantity: db::model::recipes_model::Quantity {
                amount: 1,
                unit: "cup".to_owned(),
            },
        },
        db::model::recipes_model::Ingredient {
            _id: None,
            name: "water".to_owned(),
            quantity: db::model::recipes_model::Quantity {
                amount: 1,
                unit: "cup".to_owned(),
            },
        }],
        steps: vec!["Cook rice".to_owned()],
        duration: 20,
        picture: "test".to_owned(),
     };

     db::insert_doc(rice);
     
    
}
