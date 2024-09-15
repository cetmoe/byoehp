-- Your SQL goes here
CREATE TABLE
    "users" (
        id SERIAL PRIMARY KEY,
        username VARCHAR(255) UNIQUE NOT NULL,
        email VARCHAR(255) UNIQUE NOT NULL,
        password_hash VARCHAR(255) NOT NULL,
        password_salt VARCHAR(255) NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    "characters" (
        id SERIAL PRIMARY KEY,
        character_name VARCHAR(255) UNIQUE NOT NULL,
        user_id INTEGER REFERENCES users (id) NULL,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
    );