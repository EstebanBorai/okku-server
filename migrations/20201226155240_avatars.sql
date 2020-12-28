-- Add migration script here
CREATE TABLE IF NOT EXISTS avatars (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  user_id UUID UNIQUE NOT NULL,
  large_id UUID UNIQUE NOT NULL,
  medium_id UUID UNIQUE NOT NULL,
  normal_id UUID UNIQUE NOT NULL,
  small_id UUID UNIQUE NOT NULL,
  retina_1x_id UUID UNIQUE NOT NULL,
  created_at timestamp with time zone  NOT NULL  DEFAULT current_timestamp,
  updated_at timestamp with time zone  NOT NULL  DEFAULT current_timestamp,
  FOREIGN KEY(user_id) REFERENCES users(id),
  FOREIGN KEY(large_id) REFERENCES images(id),
  FOREIGN KEY(medium_id) REFERENCES images(id),
  FOREIGN KEY(normal_id) REFERENCES images(id),
  FOREIGN KEY(small_id) REFERENCES images(id),
  FOREIGN KEY(retina_1x_id) REFERENCES images(id)
);
