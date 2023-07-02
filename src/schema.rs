// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        title -> Text,
        author -> Text,
        url_cover -> Text,
        word_count -> Integer,
        id -> Integer,
        language -> Text,
    }
}
