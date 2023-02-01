use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use dotenv;
use std::env;

use crate::schema::books;
use crate::schema::books::dsl::books as all_books;

#[derive(Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct BookData<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub published: bool,
}

impl Book {
    pub fn show(id: i32, conn: &mut MysqlConnection) -> Vec<Book> {
        all_books
            .find(id)
            .load::<Book>(conn)
            .expect("Error loading book")
    }

    pub fn all(conn: &mut MysqlConnection) -> Vec<Book> {
        all_books
            .order(books::id.desc())
            .load::<Book>(conn)
            .expect("Error loading books")
    }

    pub fn update_by_id(id: i32, connection: &mut MysqlConnection, book: BookData) -> bool {
        use crate::schema::books::dsl::{author as a, published as p, title as t};

        let BookData {
            title,
            author,
            published,
        } = book;

        diesel::update(all_books.find(id))
            .set((a.eq(author), p.eq(published), t.eq(title)))
            .execute(connection)
            .unwrap();

        true
    }
}


//  https://youtu.be/VMZdGX9wC4g?t=511 