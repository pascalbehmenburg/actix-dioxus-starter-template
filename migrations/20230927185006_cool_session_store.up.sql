-- Add up migration script here
DROP TABLE sessions;
CREATE TABLE sessions (
    session_key VARCHAR(256) PRIMARY KEY,
    user_id BIGINT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
