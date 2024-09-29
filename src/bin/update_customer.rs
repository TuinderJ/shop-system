use self::models::Customer;
use diesel::prelude::*;
use shop_system::*;
use std::env::args;

fn main() {
    // TODO add ability to change notes and customize the name change
    use self::schema::customers::dsl::{customers, name, notes};

    let customer_id = args()
        .nth(1)
        .expect("get_customer requires a customer id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let customer = diesel::update(customers.find(customer_id))
        .set(name.eq("Test"))
        .returning(Customer::as_returning())
        .get_result(connection)
        .unwrap();
    println!("Updated name to {}", customer.name);
}
