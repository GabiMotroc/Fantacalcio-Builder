-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password TEXT,
    salt TEXT,
    created_at TIMESTAMP NOT NULL,
    last_login TIMESTAMP
)