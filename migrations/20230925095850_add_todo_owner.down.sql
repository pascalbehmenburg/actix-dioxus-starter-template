-- Add down migration script here
ALTER TABLE todos DROP COLUMN owner_id BIGINT NOT NULL;