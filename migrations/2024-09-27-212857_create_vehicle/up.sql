CREATE TABLE vehicles (
  vin VARCHAR(16) NOT NULL PRIMARY KEY,
  year INTEGER NOT NULL,
  make VARCHAR NOT NULL,
  model VARCHAR NOT NULL,
  customer_id INTEGER NOT NULL,
  FOREIGN KEY (customer_id) REFERENCES customers(id)
)
