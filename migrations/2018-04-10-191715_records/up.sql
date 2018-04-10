-- Your SQL goes here
CREATE TABLE books (
  id INT(11) PRIMARY KEY,
  `title` VARCHAR(256) NOT NULL,
  author VARCHAR NOT NULL,
  publisher VARCHAR,
  year INT
);