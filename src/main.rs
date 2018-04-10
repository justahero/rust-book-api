#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate r2d2_diesel;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

use rocket_contrib::{Json, Value};

mod book;
mod db;
mod schema;

use book::{Book};

#[post("/", data = "<book>")]
fn create(book: Json<Book>, connection: db::Connection) -> Json<Book> {
    let insert = Book { id: None, ..book.into_inner() };
    Json(Book::create(insert, &connection))
}

#[get("/")]
fn list(connection: db::Connection) -> Json<Value> {
    Json(json!(Book::read(&connection)))
}

#[put("/<id>", data = "<book>")]
fn update(id: i32, book: Json<Book>, connection: db::Connection) -> Json<Value> {
    let update = Book { id: Some(id), ..book.into_inner() };
        Json(json!({
            "success": Book::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": Book::delete(id, &connection)
    }))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/book", routes![create, update, delete])
        .mount("/books", routes![list])
        .launch();
}
