-- Add down migration script here
ALTER TABLE USERS ADD COLUMN salt VARCHAR(256) NOT NULL;