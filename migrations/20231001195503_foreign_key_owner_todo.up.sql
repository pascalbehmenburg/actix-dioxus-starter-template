-- Add foreign key owner to todos BIGINT referencing users.id
ALTER TABLE todos ADD COLUMN owner BIGINT;
ALTER TABLE todos ADD CONSTRAINT todos_owner_fkey FOREIGN KEY (owner) REFERENCES users(id);