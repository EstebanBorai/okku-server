-- Database Initialization Query
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  name VARCHAR(120) NOT NULL UNIQUE,
  created_at timestamp with time zone  NOT NULL  DEFAULT current_timestamp,
  updated_at timestamp with time zone  NOT NULL  DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS secrets (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  hash VARCHAR(255) NOT NULL,
  user_id uuid,
  created_at timestamp with time zone  NOT NULL  DEFAULT current_timestamp,
  updated_at timestamp with time zone  NOT NULL  DEFAULT current_timestamp,
  FOREIGN KEY(user_id) REFERENCES users(id)
);
