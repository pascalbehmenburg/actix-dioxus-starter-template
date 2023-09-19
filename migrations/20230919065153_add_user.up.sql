-- adds users table
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(256) NOT NULL,
    email VARCHAR(256) NOT NULL,
    password VARCHAR(256) NOT NULL,
    salt VARCHAR(256) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- adds auth_tokens table for storing user auth tokens and associated device info
CREATE TABLE auth_tokens (
    id BIGSERIAL PRIMARY KEY,
    token VARCHAR(256) NOT NULL,
    device VARCHAR(256) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- adds auth_tokens_to_users n:n mapping table
-- so that users may authorize multiple devics
CREATE TABLE auth_tokens_to_users (
    auth_token_id BIGINT NOT NULL REFERENCES auth_tokens(id),
    user_id BIGINT NOT NULL REFERENCES users(id),
    PRIMARY KEY (auth_token_id, user_id)
);