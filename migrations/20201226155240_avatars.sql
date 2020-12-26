-- Add migration script here
CREATE TABLE IF NOT EXISTS avatars (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  created_at timestamp with time zone  NOT NULL  DEFAULT current_timestamp,
  updated_at timestamp with time zone  NOT NULL  DEFAULT current_timestamp,
  FOREIGN KEY(user_id) REFERENCES users(id),
  FOREIGN KEY(large_id) REFERENCES images(id),
  FOREIGN KEY(medium_id) REFERENCES images(id),
  FOREIGN KEY(normal_id) REFERENCES images(id),
  FOREIGN KEY(small_id) REFERENCES images(id),
  FOREIGN KEY(retina_1x_id) REFERENCES images(id)
);
