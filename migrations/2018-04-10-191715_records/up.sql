-- Your SQL goes here
CREATE TABLE books (
  id INTEGER PRIMARY KEY,
  `title` VARCHAR(256) NOT NULL,
  author VARCHAR NOT NULL,
  publisher VARCHAR,
  year INT
);