SELECT
  messages.id AS message_id,
  messages."content" AS message_content,
  messages.kind AS message_kind,
  messages.created_at AS message_created_at,
  messages.updated_at AS message_updated_at,
  users.id AS author_id,
  users. "name" AS author_name,
  chats.id AS chat_id
FROM
  messages
  LEFT JOIN users ON users.id = messages.author_id
  LEFT JOIN chats ON messages.chat_id = messages.chat_id
WHERE
  chat_id = $1
ORDER BY
  messages.created_at ASC;
