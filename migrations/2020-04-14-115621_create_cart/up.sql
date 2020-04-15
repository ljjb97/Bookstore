CREATE TABLE cart (
  isbn VARCHAR(13) NOT NULL,
  email VARCHAR NOT NULL,
  quantity INT NOT NULL,
  PRIMARY KEY (email, isbn)
)
