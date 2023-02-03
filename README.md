> schema should be like this 

```rs
diesel::table! {
    books (id) {
        id -> Integer,
        title -> VarChar,
        author -> VarChar,
        published -> Bool,
    }
}
```