-- Your SQL goes here
CREATE TABLE oder(
    oder_id SERIAL PRIMARY KEY,
    addrss VARCHAR NOT NULL,
    card_num VARCHAR(16) NOT NULL,
    tracking_num SERIAL,
    sttus VARCHAR,
    email VARCHAR NOT NULL
);