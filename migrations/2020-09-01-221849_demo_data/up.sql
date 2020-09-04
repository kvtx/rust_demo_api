CREATE TABLE demos
(
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  demo_text TEXT NOT NULL,
  favorite_number INT NOT NULL
)