-- TODO: add migration to alter the todo table to add a user_id column to identify a todo owner
-- ALTER TABLE todos ADD COLUMN `owner` BIGINT NOT NULL REFERENCES users(id);


-- rename table auth_tokens to sessions
ALTER TABLE auth_tokens RENAME TO sessions;

-- rename token column in sessions table to session_key
ALTER TABLE sessions RENAME COLUMN token TO session_key;

-- rename table auth_tokens_to_users to sessions_to_users
ALTER TABLE auth_tokens_to_users RENAME TO sessions_to_users;

-- rename auth_token_id column in sessions_to_users table to session_id
ALTER TABLE sessions_to_users RENAME COLUMN auth_token_id TO session_id;