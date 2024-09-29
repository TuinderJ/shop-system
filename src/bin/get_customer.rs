use self::models::Customer;
use diesel::prelude::*;
use shop_system::*;
use std::env::args;

fn main() {
    use self::schema::customers::dsl::customers;

    let customer_id = args()
        .nth(1)
        .expect("get_customer requires a customer id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let customer = customers
        .find(customer_id)
        .select(Customer::as_select())
        .first(connection)
        .optional();

    match customer {
        Ok(Some(customer)) => println!("Customer with id: {} is {}", customer.id, customer.name),
        Ok(None) => println!("Unable to find customer {}", customer_id),
        Err(_) => println!("An error occured while fetching customer {}", customer_id),
    }
}
