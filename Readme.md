Books API
---------

A simple CRUD application written in Rust, based on this [tutorial by Sean Wragg](https://medium.com/sean3z/building-a-restful-crud-api-with-rust-1867308352d8).

It uses the following tools

* [rocket](https://rocket.rs/) web framework
* [diesel](https://github.com/diesel-rs/diesel) ORM framework with Sqlite3
* [serde](https://github.com/serde-rs/serde) for JSON serialization

The application requires Rust nightly

```shell
rustup default nightly
rustup update && cargo update
```

The web application uses Diesel with a Sqlite3 database. Install the [diesel-cli](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) tool first

```shell
cargo install diesel_cli --no-default-features --features "sqlite"
```


## Setup

To use the application the Sqlite3 database has to be setup first.

```shell
# create folder
mkdir ./data
# run migrations
diesel migration run
```

To compile and run the application, execute:

```shell
cargo run
```
