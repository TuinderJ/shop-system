// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Integer,
        name -> Nullable<Text>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    vehicles (vin) {
        vin -> Text,
        year -> Nullable<Integer>,
        make -> Nullable<Text>,
        model -> Nullable<Text>,
        customer_id -> Integer,
    }
}

diesel::joinable!(vehicles -> customers (customer_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    vehicles,
);
