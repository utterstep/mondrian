table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        middle_name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        password -> Varchar,
        superuser -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
