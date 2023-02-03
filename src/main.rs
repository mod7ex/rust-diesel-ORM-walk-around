#![allow(unused)]

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use dotenv::dotenv;
use std::env;

mod schema;
mod models;

use crate::models::{ Book, BookData };

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let connection = &mut establish_connection();

    let new_title = "new book title";
    let new_author = "new book author";
    let new_published = false;

    for i in 1..11 {
        let title = format!("title {i}");
        let author = format!("author {i}");
        let published = i < 5;

        let book = BookData {
            title: &title,
            author: &author,
            published,
        };

        if insert(&book, connection) {
            println!("Success");
        } else {
            println!("Failed");
        }
    }

    println!("{:#?}", all(connection, 5));

    println!("Book [ID:2] {:#?}", show(2, connection));

    println!("Updating book [ID:1]");

    update_by_id(1, connection, BookData { 
        author: &new_author,
        title: &new_title,
        published: new_published,
    });

    println!("Book [ID:1] {:#?}", show(1, connection));

    if delete_by_id(4, connection) {
        println!("Success Post ID:4 deleted");
    } else {
        println!("Failed to drop post ID:4");
    }

    println!("All posts: {:#?}", all(connection, 10));

    let author = "author 3";
    println!("All posts with author <{}>: {:#?}", author, all_by_author(String::from(author), connection));
}

// ------------------------------------------------------------------------------------------------------------------------------------------

fn insert(book: &BookData, connection: &mut MysqlConnection) -> bool {
    use schema::books::dsl::books;

    diesel::insert_into(books)
        .values(book)
        .execute(connection)
        .is_ok()
}

fn all(connection: &mut MysqlConnection, limit: u8) -> Vec<Book> {
    use schema::books::dsl::{books, id};

    books
        .order(id.desc())
        .limit(limit as i64)
        .load::<Book>(connection)
        .expect("Error loading books")
}


pub fn show(id: i32, connection: &mut MysqlConnection) -> Vec<Book> {
    use schema::books::dsl::books;

    books
        .find(id)
        .load::<Book>(connection)
        .expect("Error loading book")
}

fn update_by_id(id: i32, connection: &mut MysqlConnection, book: BookData) -> bool {
    use schema::books::dsl::{ author as a, published as p, title as t, books };

    let BookData {
        title,
        author,
        published,
    } = book;

    let _book = books.find(id);

    diesel::update(_book)
        .set((a.eq(author), p.eq(published), t.eq(title)))
        .execute(connection)
        .unwrap();

    true
}

fn delete_by_id(id: i32, connection: &mut MysqlConnection) -> bool {
    use schema::books::dsl::books;

    if show(id, connection).is_empty() {
        return false;
    }

    let source = books.find(id);

    diesel::delete(source)
        .execute(connection)
        .is_ok()
}

fn all_by_author(author: String, connection: &mut MysqlConnection) -> Vec<Book> {
    use schema::books::dsl::{books, author as a};

    books
        .filter(a.eq(author))
        .load::<Book>(connection)
        .expect("Failed loading books")
}