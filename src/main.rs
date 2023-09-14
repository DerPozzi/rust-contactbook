use dotenv::dotenv;
use futures::TryStreamExt;
use std::error::Error;

mod database;

use crate::database::{connect_db, get_contact_by_name, Contact};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let name = "Gianluca";

    let database = connect_db().await?;
    println!("Successfully connected to database!");
    let contact_collection: mongodb::Collection<Contact> = database.collection("contacts");

    let contacts = get_contact_by_name(&contact_collection, name).await?;

    if contacts.len() == 0 {
        println!("No contact \"{}\" found", name);
    }
    for contact in contacts {
        println!("{:?}", contact)
    }

    Ok(())
}
