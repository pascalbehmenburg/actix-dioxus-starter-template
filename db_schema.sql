-- users relation
CREATE TABLE users (
	id bigserial NOT NULL UNIQUE,
	name varchar(256) NOT NULL,
	email varchar(256) NOT NULL UNIQUE,
	password varchar(256) NOT NULL,
	created_at timestamptz NOT NULL DEFAULT now(),
	updated_at timestamptz NOT NULL DEFAULT now(),
	CONSTRAINT users_pkey PRIMARY KEY (id)
);
CREATE INDEX user_email_index ON users USING hash (email);
CREATE INDEX user_id_index ON users USING hash (id);

-- session relation used for authentication and session specific data
CREATE TABLE sessions (
	id bigint NOT NULL UNIQUE,
	key varchar(64) NULL UNIQUE,
	state jsonb NOT NULL,
	CONSTRAINT sessions_pkey PRIMARY KEY (id)
);

-- todos relation
CREATE TABLE todos (
	id bigint NOT NULL UNIQUE,
	title varchar(255) NOT NULL,
	description text,
	is_done bool NOT NULL DEFAULT false,
	created_at timestamptz NOT NULL DEFAULT now(),
	updated_at timestamptz NOT NULL DEFAULT now(),
	owner bigint NOT NULL UNIQUE,
	CONSTRAINT todos_pkey PRIMARY KEY (id)
);
ALTER TABLE todos ADD CONSTRAINT todos_owner_fkey FOREIGN KEY (owner) REFERENCES users(id);