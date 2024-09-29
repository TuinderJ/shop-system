use shop_system::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut name = String::new();
    let mut notes = String::new();

    println!("What is the name of this customer?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end();

    println!("\nAre there any notes you'd like to add to {name}? (Press {EOF} when finished)\n");
    stdin().read_to_string(&mut notes).unwrap();

    let _customer = create_customer(connection, &name, &notes);
    println!("\nSaved {name} to the database");
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
