table! {
    task_assignments (id) {
        id -> Int4,
        task_id -> Int4,
        user_id -> Nullable<Int4>,
        start_time -> Timestamp,
        duration -> Int4,
        completed -> Bool,
        priority -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        author_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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

joinable!(task_assignments -> tasks (task_id));
joinable!(task_assignments -> users (user_id));
joinable!(tasks -> users (author_id));

allow_tables_to_appear_in_same_query!(
    task_assignments,
    tasks,
    users,
);
