-- drop indecies
DROP INDEX user_email_index;
DROP INDEX user_id_index;

-- drop tables in reverse order of how they were craeted because of foreign key constraints
DROP TABLE todos;
DROP TABLE sessions;
DROP TABLE users;