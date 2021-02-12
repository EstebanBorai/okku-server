SELECT
  STRING_AGG(user_id::text, ',') AS users,
  chat_id
FROM
  chats_users
WHERE
  chat_id IN (
    SELECT
      chat_id FROM chats_users
    WHERE
      user_id = $1)
GROUP BY
  chat_id
ORDER BY
  chat_id ASC
