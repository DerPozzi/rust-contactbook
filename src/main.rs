use dotenv::dotenv;
use futures::stream::TryStreamExt;
use mongodb::bson::*;
use mongodb::options::{ClientOptions, FindOptions};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Contact {
    name: String,
    last_name: String,
    phone: String,
    email: String,
    birthday: String,
    notes: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let database = connect_db().await?;
    let contact_collection: mongodb::Collection<Contact> = database.collection("contacts");

    /*

    let filter = doc! {"Name": "Gianluca"};

    let mut cursor = contact_collection
        .find(filter, FindOptions::builder().build())
        .await?;

    while let Some(contact) = cursor.try_next().await? {
        println!("{:?}", contact);
    }

    */

    Ok(())
}

async fn connect_db() -> Result<mongodb::Database, Box<dyn Error>> {
    let db_url = env::var("MONGODB_URL")?;
    let db = env::var("MONGO_DB")?;
    let db_options = ClientOptions::parse(db_url).await?;
    let db_client = Client::with_options(db_options)?;

    let database = db_client.database(db.as_str());
    database.run_command(doc! {"ping": 1}, None).await?;

    Ok(database)
}
