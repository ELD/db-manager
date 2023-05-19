-- Add migration script here
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE
);

CREATE TABLE "post" (
    id SERIAL PRIMARY KEY,
    author INT,
    title TEXT
);
