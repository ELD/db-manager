-- Your SQL goes here
CREATE TABLE todo (
    id SERIAL PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    done BOOL NOT NULL DEFAULT FALSE
);
