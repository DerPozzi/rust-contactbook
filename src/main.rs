extern crate dotenv;

use serde::{Deserialize, Serialize};
use futures::stream::TryStreamExt;
use mongodb::bson::*;
use mongodb::options::{ClientOptions, FindOptions};
use mongodb::{self, Client, Collection};
use std::env;
use std::error::Error;
use tokio;
use dotenv::dotenv;

#[derive(Debug,Deserialize, Serialize)]
struct Contact {
    name: String,
    last_name: String,
    phone: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();

    let db_url = env::var("MONGODB_URL")?; 
    let client = connect_to_db(db_url.clone()).await?;
    let collection = get_db_collection(client.clone(), "contact_book", "contacts")?;
    let mut cursor = get_all_contacts(collection).await?;

   while let Some(contact) = cursor.try_next().await? {
        println!("Name: {0}, {1} \nPhone: {2}", contact.last_name, contact.name, contact.phone);
    }

    

    Ok(())
}

async fn get_all_contacts (collection: Collection<Contact>) -> Result<mongodb::Cursor<Contact>, Box<dyn Error>> {
    let find_options = FindOptions::builder().sort(doc! {"last_name": 1}).build();

    let cursor = collection.find(None, find_options).await?;

    Ok(cursor)
}

fn get_db_collection (client: Client, database: &str, collection: &str) -> Result<Collection<Contact>, Box<dyn Error>> {
    let database = client.database(database);
    let collection = database.collection::<Contact>(collection);

    Ok(collection)
}

async fn connect_to_db (db_url: String) -> Result<Client, Box<dyn Error>> {
    let client_options = ClientOptions::parse(db_url).await?;
    let client : Client = Client::with_options(client_options)?;

    Ok(client)
}
