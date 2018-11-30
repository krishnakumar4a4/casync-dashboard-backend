-- Your SQL goes here
CREATE TABLE chunk (
       id SERIAL PRIMARY KEY NOT NULL,
       index_id INT NOT NULL,
       name TEXT NOT NULL,
       size INT NOT NULL,
       creation_time TEXT NOT NULL,
       accessed_time TEXT NOT NULL,
       tags integer ARRAY,
       stats_download_count INT NOT NULL
);

CREATE TABLE tag (
       id SERIAL PRIMARY KEY NOT NULL,
       name TEXT NOT NULL,
       creation_time timestamp with time zone NOT NULL,
       accessed_time timestamp with time zone NOT NULL
);
