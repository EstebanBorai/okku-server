-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  name VARCHAR(120) NOT NULL UNIQUE,
  created_at timestamp with time zone  NOT NULL  DEFAULT current_timestamp,
  updated_at timestamp with time zone  NOT NULL  DEFAULT current_timestamp
);
