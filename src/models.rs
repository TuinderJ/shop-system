use diesel::prelude::*;
use crate::schema::customers;
use crate::schema::vehicles;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub notes: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = customers)]
pub struct NewCustomer<'a> {
    pub name: &'a str,
    pub notes: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::vehicles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Vehicle {
    pub vin: String,
    pub year: i32,
    pub make: String,
    pub model: String,
    pub customer_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = vehicles)]
pub struct NewVehicle<'a> {
    pub vin: &'a str,
    pub year: i32,
    pub make: &'a str,
    pub model: &'a str,
    pub customer_id: i32,
}
