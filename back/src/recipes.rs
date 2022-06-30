use bson::oid::ObjectId;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Recipes {
    _id: ObjectId,
    name: String,
    ingredients: Vec<Ingredient>,
    steps: Vec<String>,
    duration: u32,
    picture: String,
}

struct Ingredient {
    _id: ObjectId,
    name: String,
    quantity: Quantity,
}

struct Quantity {
    amount: u32,
    unit: String,
}