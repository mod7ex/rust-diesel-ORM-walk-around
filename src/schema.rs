// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Integer,
        title -> Char,
        author -> Char,
        published -> Bool,
    }
}
