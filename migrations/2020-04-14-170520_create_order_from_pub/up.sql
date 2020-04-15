-- Your SQL goes here
CREATE TABLE order_from_pub(
    order_id SERIAL PRIMARY KEY,
    cost INTEGER NOT NULL,
    tracking_number SERIAL,
    sttus VARCHAR,
    pub_name VARCHAR
);