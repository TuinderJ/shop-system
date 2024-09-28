use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::customers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub notes: String,
}

pub struct Vehicle {
    pub vin: String,
    pub year: i32,
    pub make: String,
    pub model: String,
    pub customer_id: i32,
}
