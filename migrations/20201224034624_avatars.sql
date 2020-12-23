-- Add migration script here
CREATE TABLE IF NOT EXISTS avatars (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  image bytea,
  user_id uuid UNIQUE,
  mime_type VARCHAR(120),
  created_at timestamp with time zone  NOT NULL  DEFAULT current_timestamp,
  updated_at timestamp with time zone  NOT NULL  DEFAULT current_timestamp,
  FOREIGN KEY(user_id) REFERENCES users(id)
);
