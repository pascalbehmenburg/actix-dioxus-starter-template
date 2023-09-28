-- Add down migration script here
ALTER TABLE sessions DROP CONSTRAINT sessions_user_id_fkey;