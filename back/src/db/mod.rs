use mongodb::{Client, options::{ClientOptions, ResolverConfig}, Database, Collection, bson};
use std::env;
use dotenv;
use tokio;
pub mod model;

// mod request;

#[tokio::main]
pub async fn connect() -> mongodb::error::Result<Database> {
    dotenv::dotenv().ok();
    // Load the MongoDB connection string from an environment variable:
    let mongo_uri = env::var("MONGO_DB_URI").expect("MONGO_DB_URI is not found.");
    let mongo_db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME is not found.");

    // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
   ClientOptions::parse_with_resolver_config(&mongo_uri, ResolverConfig::cloudflare())
      .await?;
    let client = Client::with_options(options)?;
   
    let database = client.database(mongo_db_name.as_str());

    println!("MongoDB Connected!");
    // println!("Databases:");
    // for name in client.list_database_names(None, None).await? {
    //    println!("- {}", name);
    // }
    Ok(database)
}

fn init() -> Database {
    let db: Database = match connect() {
        Ok(database) => database,
        Err(error) => {
            panic!("Cannot connect to instance:: {:?}", error)
        }
    };
   db
}

pub fn collection_recipe()-> Collection<model::recipes_model::Recipes>{
    let db: Database = init();
    let collection = db.collection("recipe");
    collection
}

// pub fn collection_ingredient()-> Collection<model::recipes_model::Ingredient>{
//     let db: Database = init();
//     let collection = db.collection("ingredient");
//     collection
// }

pub async fn insert_doc( doc: model::recipes_model::Recipes
) -> mongodb::error::Result<String> {
    let collection = init().collection("recipe");

    let insert_one_result = collection
        .insert_one(
            doc,
            None,
        )
        .await?;
    print!("{}", insert_one_result.inserted_id.to_string());
    Ok(insert_one_result.inserted_id.to_string())
}