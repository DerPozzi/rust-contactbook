extern crate dotenv;

use serde::{Deserialize, Serialize};
use futures::stream::TryStreamExt;
use mongodb::bson::*;
use mongodb::options::{ClientOptions, FindOptions};
use mongodb::{self, Client, Collection};
use std::{env};
use std::io::stdin;
use std::error::Error;
use tokio;
use dotenv::dotenv;

#[derive(Debug,Deserialize, Serialize)]
struct Contact {
    name: String,
    last_name: String,
    birthday: String,
    phone: String,
    email: String,
    notes: String,
}

async fn main_menu(db_url: String) {
    let my_client = connect_to_db(db_url).await.expect("{msg}");
    let my_collection = get_db_collection(my_client.clone(), "contact_book", "contacts").expect("{msg}");

    loop {
        let mut input = String::new();
        println!("(1) Add contact \n(2) Edit contact \n(3) Show contacts \n(4) Search for contact \n(5) Delete contact \n(0) Exit program");
        println!("Which action do you want to perform ? ");

        stdin().read_line(&mut input).expect("ERROR: {msg}");
        let input: u8 = input.trim().parse().unwrap();

        match input {
            0 => {
                println!("Exiting program..."); 
                break;
            },
            1 => println!("add_contact();"),
            2 => println!("edit_contact();"),
            3 => show_all_contacts(my_collection.clone()).await.expect("{msg}"),
            4 => show_specific_contact(my_collection.clone()).await.expect("ERROR: {msg}"),
            5 => println!("delete_contact();"),
            6..=u8::MAX => println!("ERROR: Input didn't match a task"),
        }
    }
}

async fn show_all_contacts(collection: Collection<Contact>) -> Result<(), Box<dyn Error>> {
    let mut cursor = get_all_contacts(collection.clone()).await?;
    let mut i = 1;

    while let Some(contact) = cursor.try_next().await? {
        println!("{}.\t {} {}", i, contact.name, contact.last_name);
        i += 1;
    }
    Ok(())
}

async fn show_specific_contact (collection: Collection<Contact>) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut searched_name = String::new();
    let filter: Document;

    loop {
        println!("Do you want to search after name (1) or lastname (2)? ");
        stdin().read_line(&mut input).expect("ERROR: {msg}");

        let input: u8 = input.trim().parse().unwrap();

    match input {
        1 => {
            println!("Enter the name of the contact:");
            stdin().read_line(&mut searched_name).expect("ERROR: {msg}");
            searched_name = searched_name.trim().parse().unwrap();
            filter = doc! {"name": searched_name};
            break;
        },
        2 => {
            println!("Enter the name of the contact:");
            stdin().read_line(&mut searched_name).expect("ERROR: {msg}");
            searched_name = searched_name.trim().parse().unwrap();
            filter = doc! {"last_name": searched_name};
            break;
        },
        _ => {
            println!("Oops, something went wrong, try again... ");
        }
    }}

        

    let mut result = get_specific_contact(collection, filter).await?;
    let mut i = 1;

    while let Some(contact) = result.try_next().await? {
        println!("{}.\t{} {}", i, contact.name, contact.last_name);
        println!("\tPhone: {}", contact.phone);
        println!("\tEmail: {}", contact.email);
        println!("\tBirthday: {}", contact.birthday);
        println!("\tNotes: {}", contact.notes);
        println!();
        i += 1;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    dotenv().ok();

    let db_url = env::var("MONGODB_URL")?; 
    
    main_menu(db_url).await;

    Ok(())
}

async fn get_all_contacts (collection: Collection<Contact>) -> Result<mongodb::Cursor<Contact>, Box<dyn Error>> {
    let find_options = FindOptions::builder().sort(doc! {"name": 1}).build();
    let cursor = collection.find(None, find_options).await?;

    Ok(cursor)
}

async fn get_specific_contact (collection: Collection<Contact>, filter: Document) -> Result<mongodb::Cursor<Contact>, Box<dyn Error>> {
    let cursor = collection.find(filter, None).await?;
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
