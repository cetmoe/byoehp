// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Int4,
        #[max_length = 255]
        character_name -> Varchar,
        user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 255]
        password_salt -> Varchar,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(characters -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(characters, users,);
