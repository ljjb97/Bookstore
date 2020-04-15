-- Your SQL goes here
CREATE TABLE order_contents(
    order_id INTEGER NOT NULL,
    isbn VARCHAR(13) NOT NULL,
    direction VARCHAR(3) NOT NULL,
    quantity INTEGER NOT NULL,
    PRIMARY KEY(order_id, isbn, direction)
);