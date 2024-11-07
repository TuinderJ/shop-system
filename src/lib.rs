pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::{NewCustomer, Customer};
// use self::models::{NewCustomer, Customer, NewVehicle, Vehicle};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_customer(connection: &mut SqliteConnection, name: &str, notes: &str) -> Customer {
    use crate::schema::customers;

    let new_customer = NewCustomer { name, notes };

    diesel::insert_into(customers::table)
        .values(&new_customer)
        .returning(Customer::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
}
