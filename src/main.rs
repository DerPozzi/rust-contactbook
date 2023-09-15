use dotenv::dotenv;
use std::error::Error;
use text_io::scan;

mod database;

use crate::database::{connect_to_db, find_contact_by_name, insert_new_contact, Contact};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let database = connect_to_db().await?;
    println!("Successfully connected to database!");
    let contact_collection: mongodb::Collection<Contact> = database.collection("contacts");

    // TODO: Mainmenu
    // TODO: Edit contact
    // TODO: Delete contact
    // TODO: Show all contacts

    Ok(())
}

async fn add_new_contact(collection: &mongodb::Collection<Contact>) -> Result<(), Box<dyn Error>> {
    let mut new_contact = Contact {
        name: "".to_owned(),
        last_name: "".to_owned(),
        phone: "".to_owned(),
        email: "".to_owned(),
        birthday: "".to_owned(),
        notes: "".to_owned(),
    };
    println!("Add new Contact");
    println!("What's the first and lastname?");
    scan!("{} {}", new_contact.name, new_contact.last_name);

    println!("What's the phone number?");
    scan!("{}\n", new_contact.phone);

    println!("What's the email adress?");
    scan!("{}\n", new_contact.email);

    println!("When's the birthday?");
    scan!("{}\n", new_contact.birthday);

    println!("What are the notes?");
    scan!("{}\n", new_contact.notes);

    insert_new_contact(collection, new_contact).await?;
    Ok(())
}
