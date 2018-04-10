use diesel;
use diesel::prelude::*;
use schema::books;

#[table_name = "books"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Book {
    pub id: Option<i32>,
    pub title: String,
    pub author: String,
    pub publisher: String,
    pub year: i32
}

impl Book {
    pub fn create(book: Book, connection: &SqliteConnection) -> Book {
        diesel::insert_into(books::table)
            .values(&book)
            .execute(connection)
            .expect("Error creating new book");

        books::table.order(books::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &SqliteConnection) -> Vec<Book> {
        books::table.order(books::id).load::<Book>(connection).unwrap()
    }

    pub fn update(id: i32, book: Book, connection: &SqliteConnection) -> bool {
        diesel::update(books::table.find(id)).set(&book).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &SqliteConnection) -> bool {
        diesel::delete(books::table.find(id)).execute(connection).is_ok()
    }
}
