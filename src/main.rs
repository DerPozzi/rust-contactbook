use dotenv::dotenv;
use std::error::Error;
use text_io::scan;

mod database;

use crate::database::{connect_db, get_contact_by_name, Contact};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let database = connect_db().await?;
    println!("Successfully connected to database!");
    let contact_collection: mongodb::Collection<Contact> = database.collection("contacts");

    let name: String;
    println!("Enter a name to search for in your contacts");
    scan!("{}", name);

    let contacts = get_contact_by_name(&contact_collection, name.as_str()).await?;

    println!(
        "Found {} contacts with the name \"{}\"",
        contacts.len(),
        name
    );
    for contact in contacts {
        println!("Name:\t\t{}", contact.name);
        println!("Lastname:\t{}", contact.last_name);
        println!("Phone:\t\t{}", contact.phone);
        println!("Email:\t\t{}", contact.email);
        println!("Birthday:\t{}", contact.birthday);
        println!("Notes:\t\t{}", contact.notes);
        println!()
    }

    Ok(())
}
