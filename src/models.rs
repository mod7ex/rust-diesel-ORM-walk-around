use diesel::prelude::*;
use crate::schema::books;

#[derive(Queryable, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = books)]
pub struct BookData<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub published: bool,
}