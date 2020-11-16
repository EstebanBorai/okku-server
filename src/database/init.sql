-- Database Initialization Query
CREATE TABLE users (
  id uuid,
  username VARCHAR(120) NOT NULL,
  first_name VARCHAR(120),
  surname VARCHAR(120),
  email VARCHAR(100),
  avatar_id uuid,
  PRIMARY KEY(id)
);

CREATE TABLE user_avatar (
  id uuid,
  mime_type CHARACTER VARYING(255) NOT NULL,
  file_name CHARACTER VARYING(255) NOT NULL,
  bytes BYTEA NOT NULL,
  FOREIGN KEY(id) REFERENCES users(avatar_id)
);
