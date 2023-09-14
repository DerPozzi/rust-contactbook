use futures::TryStreamExt;
use mongodb::options::{ClientOptions, FindOptions};
use mongodb::{bson::*, Client};
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Contact {
    name: String,
    last_name: String,
    phone: String,
    email: String,
    birthday: String,
    notes: String,
}

pub async fn connect_db() -> Result<mongodb::Database, Box<dyn Error>> {
    let db_url = env::var("MONGODB_URL")?;
    let db = env::var("MONGO_DB")?;
    let db_options = ClientOptions::parse(db_url).await?;
    let db_client = Client::with_options(db_options)?;

    let database = db_client.database(db.as_str());
    database.run_command(doc! {"ping": 1}, None).await?;

    Ok(database)
}

pub async fn get_contact_by_name(
    collection: &mongodb::Collection<Contact>,
    name: &str,
) -> Result<Vec<Contact>, Box<dyn Error>> {
    let filter = doc! {"Name": name};
    let sort_option = FindOptions::builder()
        .allow_partial_results(true)
        .sort(doc! {"LastName" : 1})
        .build();
    let cursor = collection.find(filter, sort_option).await?;
    Ok(cursor.try_collect().await?)
}
