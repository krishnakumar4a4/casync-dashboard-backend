-- Your SQL goes here
CREATE TABLE chunks (
       id SERIAL PRIMARY KEY NOT NULL,
       index TEXT NOT NULL,
       name TEXT NOT NULL,
       size INT NOT NULL
)
