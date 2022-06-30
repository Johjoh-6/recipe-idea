use mongodb::{Client, options::{ClientOptions, ResolverConfig}, Database};
use std::env;
use dotenv;
use tokio;

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

    Ok(database)
}