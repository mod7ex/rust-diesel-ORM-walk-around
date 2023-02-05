use diesel::prelude::*;
use crate::schema::books;

#[derive(Serialize, Queryable, Debug, Clone)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub published: bool,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = books)]
pub struct BookData<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub published: bool,
}

impl Book {
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
}
