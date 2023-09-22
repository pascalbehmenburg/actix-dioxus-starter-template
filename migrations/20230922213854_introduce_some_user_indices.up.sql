-- Add up migration script here
CREATE INDEX user_email_index ON users USING HASH(email);
CREATE INDEX user_id_index ON users USING HASH(id);
