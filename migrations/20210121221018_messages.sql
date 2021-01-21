-- Add migration script here
CREATE TABLE IF NOT EXISTS messages (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  content TEXT NOT NULL,
  kind VARCHAR(32) NOT NULL,
  author_id UUID NOT NULL,
  chat_id UUID NOT NULL,
  file_id UUID,
  created_at TIMESTAMP WITH TIME ZONE  NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE  NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(author_id) REFERENCES users(id),
  FOREIGN KEY(chat_id) REFERENCES chats(id),
  FOREIGN KEY(file_id) REFERENCES files(id)
);
