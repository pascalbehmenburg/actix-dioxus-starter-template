-- Add down migration script here
ALTER TABLE sessions DROP COLUMN user_id;
ALTER TABLE sessions DROP COLUMN last_visited_at;
ALTER TABLE sessions DROP COLUMN logged_in_at;
ALTER TABLE todos DROP CONSTRAINT todos_owner_id_fkey;