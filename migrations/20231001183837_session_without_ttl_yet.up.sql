-- Add up migration script here
DROP TABLE sessions;
CREATE TABLE sessions
(
    id    BIGSERIAL PRIMARY KEY,
    key   VARCHAR(64),
    state JSONB NOT NULL
);