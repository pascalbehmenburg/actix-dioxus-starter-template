-- Add up migration script here
-- adjust session_token size to 512
ALTER TABLE sessions ALTER COLUMN session_key TYPE VARCHAR(512);