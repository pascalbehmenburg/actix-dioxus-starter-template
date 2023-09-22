-- Add down migration script here
-- adjust session_token size to 256
ALTER TABLE sessions ALTER COLUMN session_key TYPE VARCHAR(256);