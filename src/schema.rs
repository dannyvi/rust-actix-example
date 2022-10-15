// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        created_by -> Text,
        created_at -> Timestamp,
        updated_by -> Text,
        updated_at -> Timestamp,
    }
}
