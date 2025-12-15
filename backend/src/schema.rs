// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        post_id -> Int4,
        author_name -> Text,
        author_hash -> Text,
        password_hash -> Text,
        content -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        category_id -> Int4,
        author_name -> Text,
        author_hash -> Text,
        password_hash -> Text,
        title -> Text,
        content -> Text,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(posts -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(categories, comments, posts,);
