use self::models::*;
use diesel::prelude::*;
use shop_system::*;

fn main() {
    use self::schema::customers::dsl::*;

    let connection = &mut establish_connection();
    let results = customers
        .limit(5)
        .select(Customer::as_select())
        .load(connection)
        .expect("Error loading customers")

    println!("Displaying {} customers.", results.len());
    for customer in results {
        println!("{}", customer.name);
        println!("------------\n");
        println!("{}", customer.notes);
    }
}
