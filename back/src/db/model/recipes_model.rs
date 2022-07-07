use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Recipes {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
   pub _id: Option<ObjectId>,
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<String>,
    pub duration: u32,
    pub picture: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub name: String,
    pub quantity: Quantity,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quantity {
    pub amount: u32,
    pub unit: String,
}