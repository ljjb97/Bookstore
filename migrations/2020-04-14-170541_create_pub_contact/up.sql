-- Your SQL goes here
CREATE TABLE pub_contact(
    email VARCHAR PRIMARY KEY,
    fname VARCHAR NOT NULL,
    lname VARCHAR NOT NULL,
    phone_num VARCHAR(11) NOT NULL,
    role VARCHAR NOT NULL,
    pub_name VARCHAR NOT NULL
);