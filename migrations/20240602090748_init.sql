-- Add migration script here
CREATE TABLE todos
(
    id SERIAL PRIMARY KEY,
    text TEXT NOT NULL,
    status INTEGER NOT NULL DEFAULT 0
);
