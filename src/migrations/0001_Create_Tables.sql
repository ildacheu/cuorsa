CREATE TABLE participants (
  id UUID DEFAULT gen_random_uuid () NOT NULL,
  email VARCHAR(255) NOT NULL,
  num VARCHAR(255) NOT NULL,
  prenum VARCHAR(255) NOT NULL,
  agen_vehichel BOOLEAN NOT NULL,
  email_confirmed BOOLEAN NOT NULL
)
