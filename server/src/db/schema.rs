diesel::table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Int8,
        email -> Text,
        password -> Text,
        firstname -> Text,
        lastname -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
