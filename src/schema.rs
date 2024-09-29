// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Integer,
        name -> Text,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    vehicles (vin) {
        vin -> Text,
        year -> Integer,
        make -> Text,
        model -> Text,
        customer_id -> Integer,
    }
}

diesel::joinable!(vehicles -> customers (customer_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    vehicles,
);
