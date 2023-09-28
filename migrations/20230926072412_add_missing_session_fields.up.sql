-- actix identity requires these fields of a hashmap to be stored inside the session store
-- so that one may use the identity middleware
-- pub(crate) const ID_KEY: &str = "actix_identity.user_id";
-- pub(crate) const LAST_VISIT_UNIX_TIMESTAMP_KEY: &str = "actix_identity.last_visited_at";
-- pub(crate) const LOGIN_UNIX_TIMESTAMP_KEY: &str = "actix_identity.logged_in_at";
-- since all migrations will be merged to one file when moving to a stable version of this template
-- let's just truncate data here for now
DROP TABLE sessions_to_users;
TRUNCATE sessions;
TRUNCATE users;
TRUNCATE todos;
ALTER TABLE sessions ADD COLUMN user_id BIGINT NOT NULL;
ALTER TABLE sessions ADD COLUMN last_visited_at BIGINT NOT NULL;
ALTER TABLE sessions ADD COLUMN logged_in_at BIGINT NOT NULL;

-- also noticed todos is missing a foreign key constraint on the owner_id column to users.id
ALTER TABLE todos ADD CONSTRAINT todos_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE;