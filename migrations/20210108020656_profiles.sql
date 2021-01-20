-- Add migration script here
CREATE TABLE IF NOT EXISTS profiles (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  first_name VARCHAR(64),
  email VARCHAR(256) NOT NULL UNIQUE,
  surname VARCHAR(64),
  birthday DATE,
  bio VARCHAR(256),
  user_id UUID NOT NULL UNIQUE,
  avatar_id UUID UNIQUE,
  created_at TIMESTAMP WITH TIME ZONE  NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE  NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(user_id) REFERENCES users(id),
  FOREIGN KEY(avatar_id) REFERENCES avatars(id)
);
