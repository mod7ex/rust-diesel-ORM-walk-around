-- Your SQL goes here

CREATE TABLE books (
  id SERIAL PRIMARY KEY,
  title CHAR(20) NOT NULL,
  author CHAR(20) NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
)