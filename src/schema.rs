// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Integer,
        title -> VarChar,
        author -> VarChar,
        published -> Bool,
    }
}
