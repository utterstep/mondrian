-- Your SQL goes here
CREATE TABLE USERS (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL DEFAULT '',
    last_name VARCHAR NOT NULL DEFAULT '',
    middle_name VARCHAR NOT NULL DEFAULT '',
    email VARCHAR NOT NULL UNIQUE,
    phone VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL, -- bcrypt hash
    superuser BOOL NOT NULL DEFAULT 'f',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('users');
