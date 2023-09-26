-- Add up migration script here
TRUNCATE todos;
ALTER TABLE todos ADD COLUMN owner_id BIGINT NOT NULL;