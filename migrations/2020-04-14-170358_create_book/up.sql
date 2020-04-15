-- Your SQL goes here
CREATE TABLE book(
    isbn VARCHAR(13) PRIMARY KEY,
    author_fname VARCHAR NOT NULL,
    author_lname VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    genre VARCHAR NOT NULL,
    page_count INTEGER NOT NULL,
    price REAL NOT NULL,
    stock SMALLINT NOT NULL,
    prcnt_of_sale REAL NOT NULL,
    pub_name VARCHAR NOT NULL
);