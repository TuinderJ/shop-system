CREATE TABLE vehicles (
  vin VARCHAR(16) NOT NULL PRIMARY KEY,
  year INTEGER,
  make VARCHAR,
  model VARCHAR,
  customer_id INTEGER NOT NULL,
  FOREIGN KEY (customer_id) REFERENCES customers(id)
)
