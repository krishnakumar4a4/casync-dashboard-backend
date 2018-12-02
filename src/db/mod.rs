use dotenv::dotenv;
use std::env;

pub mod models;

use postgres::{Connection, TlsMode};
use chrono::{DateTime, Utc};
use ds;

pub fn establish_connection() -> Connection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    Connection::connect(database_url.clone(), TlsMode::None)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_tables() {
    let conn = establish_connection();
    conn.execute("CREATE TABLE IF NOT EXISTS chunk (
       id SERIAL PRIMARY KEY NOT NULL,
       index_id INT NOT NULL,
       name TEXT NOT NULL,
       size INT NOT NULL,
       creation_time timestamp with time zone NOT NULL,
       accessed_time timestamp with time zone NOT NULL,
       tags integer ARRAY,
       stats_download_count INT NOT NULL)", &[]).expect("Table chunk doesn't exist and could not create it");
    conn.execute("CREATE TABLE IF NOT EXISTS tag (
       id SERIAL PRIMARY KEY NOT NULL,
       name TEXT NOT NULL,
       creation_time timestamp with time zone NOT NULL,
       accessed_time timestamp with time zone NOT NULL
)", &[]).expect("Table tag doesn't exist and could not create it");
    conn.execute("CREATE TABLE IF NOT EXISTS index (
       id SERIAL PRIMARY KEY NOT NULL,
       name TEXT NOT NULL,
       PATH TEXT NOT NULL,
       chunks integer ARRAY,
       creation_time timestamp with time zone NOT NULL,
       accessed_time timestamp with time zone NOT NULL,
       stats_confirmed_download_count INT NOT NULL,
       stats_anonymous_download_count INT NOT NULL)", &[]).expect("Table index doesn't exist and could not create it");
}

pub fn drop_tables() {
    let conn = establish_connection();
    conn.execute("DROP TABLE IF EXISTS chunk",&[]).expect("Could not drop table chunk");
    conn.execute("DROP TABLE IF EXISTS tag",&[]).expect("Could not drop table tag");
    conn.execute("DROP TABLE IF EXISTS index", &[]).expect("Could not drop table index");
}

pub fn load_seed_data() {
    let conn = establish_connection();

    // Insert chunks to chunk table
    let chunk1 = models::Chunk{
        id: 1,
        index_id: 1,
        name: "Chunk1".to_owned(),
        size: 11,
        creation_time: Utc::now(),
        accessed_time: Utc::now(),
        tags: Some(vec![1,2]),
        stats_download_count: 234
    };
    let chunk2 = models::Chunk{
        id: 2,
        index_id: 1,
        name: "Chunk2".to_owned(),
        size: 12,
        creation_time: Utc::now(),
        accessed_time: Utc::now(),
        tags: Some(vec![1]),
        stats_download_count: 23
    };
    conn.execute("insert into chunk(index_id, name, size,
                   creation_time, accessed_time, tags, stats_download_count)
                   values($1,$2,$3,$4,$5,$6,$7);",
                 &[&chunk1.index_id,&chunk1.name,&chunk1.size,
                   &chunk1.creation_time, &chunk1.accessed_time, &chunk1.tags,
                   &chunk1.stats_download_count]).expect("Could not insert seed data");

    conn.execute("insert into chunk(index_id, name, size,
                   creation_time, accessed_time, tags, stats_download_count)
                   values($1,$2,$3,$4,$5,$6,$7);",
                 &[&chunk2.index_id,&chunk2.name,&chunk2.size,
                   &chunk2.creation_time, &chunk2.accessed_time, &chunk2.tags,
                   &chunk2.stats_download_count]).expect("Could not insert seed data");

    // Insert tags into tag table
    let tag1 = models::Tag {
        id: 1,
        name: "rel1".to_owned(),
        creation_time: Utc::now(),
        accessed_time: Utc::now()
    };
    conn.execute("INSERT INTO tag VALUES ($1, $2, $3, $4)",
                 &[&tag1.id, &tag1.name, &tag1.creation_time, &tag1.accessed_time])
        .expect("Could not insert seed data");

    //Insert indexes into index table
    let index1 = models::Index{
        id: 1,
        name: "index1.caibx".to_owned(),
        path: "./".to_owned(),
        chunks: Some(vec![1,2]),
        creation_time: Utc::now(),
        accessed_time: Utc::now(),
        stats_confirmed_download_count: 12,
        stats_anonymous_download_count: 13
    };
    conn.execute("INSERT INTO index(name, path, chunks, creation_time, accessed_time,
                  stats_confirmed_download_count, stats_anonymous_download_count)
                  VALUES ($1, $2, $3, $4, $5, $6, $7)",
                 &[&index1.name, &index1.path, &index1.chunks, &index1.creation_time,
                   &index1.accessed_time, &index1.stats_confirmed_download_count,
                   &index1.stats_anonymous_download_count])
        .expect("Could not insert seed data");

    let index2 = models::Index{
        id: 2,
        name: "index2.caibx".to_owned(),
        path: "./".to_owned(),
        chunks: Some(vec![1]),
        creation_time: Utc::now(),
        accessed_time: Utc::now(),
        stats_confirmed_download_count: 1,
        stats_anonymous_download_count: 131
    };
    conn.execute("INSERT INTO index(name, path, chunks, creation_time, accessed_time,
                  stats_confirmed_download_count, stats_anonymous_download_count)
                  VALUES ($1, $2, $3, $4, $5, $6, $7)",
                 &[&index2.name, &index2.path, &index2.chunks, &index2.creation_time,
                   &index2.accessed_time, &index2.stats_confirmed_download_count,
                   &index2.stats_anonymous_download_count])
        .expect("Could not insert seed data");
}

pub fn chunks_for_index_id(index_id: i32) -> Vec<models::Chunk> {
    let conn = establish_connection();
    let mut chunks: Vec<models::Chunk> = Vec::new();

    match conn.query("SELECT id,index_id,name,size,creation_time,accessed_time,tags,stats_download_count FROM chunk where index_id = $1", &[&index_id]) {
        Ok(rows) => {
            for row in rows.iter() {
                let chunk = models::Chunk {
                    id: row.get(0),
                    index_id: row.get(1),
                    name: row.get(2),
                    size: row.get(3),
                    creation_time: row.get(4),
                    accessed_time: row.get(5),
                    tags: row.get(6),
                    stats_download_count: row.get(7)
                };
                chunks.push(chunk);
            };
        },
        Err(e) => {
            println!("Error getting chunks for index {}", index_id);
        }
    };
    chunks
}


pub fn chunks_for_ids(ids: Vec<i32>) -> Vec<models::Chunk> {
    let conn = establish_connection();
    let mut chunks: Vec<models::Chunk> = Vec::new();

    match conn.query("SELECT id,index_id,name,size,creation_time,accessed_time,tags,stats_download_count FROM chunk where id = ANY($1)", &[&ids]) {
        Ok(rows) => {
            for row in rows.iter() {
                let chunk = models::Chunk {
                    id: row.get(0),
                    index_id: row.get(1),
                    name: row.get(2),
                    size: row.get(3),
                    creation_time: row.get(4),
                    accessed_time: row.get(5),
                    tags: row.get(6),
                    stats_download_count: row.get(7)
                };
                chunks.push(chunk);
            };
        },
        Err(e) => {
            println!("Error getting chunks for ids, error {}", e);
        }
    };
    chunks
}

pub fn tags_for_ids(ids: Vec<i32>) -> Vec<models::Tag> {
    let conn = establish_connection();
    let mut tags: Vec<models::Tag> = Vec::new();

    match conn.query("SELECT id, name, creation_time, accessed_time FROM tag WHERE id = ANY($1)", &[&ids]) {
        Ok(rows) => {
            for row in rows.iter() {
                let tag = models::Tag {
                    id: row.get(0),
                    name: row.get(1),
                    creation_time: row.get(2),
                    accessed_time: row.get(3)
                };
                tags.push(tag);
            }
        },
        Err(e) => {
            println!("Error getting tags for ids, error {}",e);
        }
    };
    tags
}

pub fn indexes_for_ids(ids: Vec<i32>) -> Vec<models::Index> {
    let conn = establish_connection();
    let mut indexes: Vec<models::Index> = Vec::new();

    match conn.query("SELECT id, name, path, chunks, creation_time, accessed_time,
                     stats_confirmed_download_count, stats_anonymous_download_count
                     FROM index WHERE id = ANY($1)", &[&ids]) {
        Ok(rows) => {
            for row in rows.iter() {
                let index = models::Index {
                    id: row.get(0),
                    name: row.get(1),
                    path: row.get(2),
                    chunks: row.get(3),
                    creation_time: row.get(4),
                    accessed_time: row.get(5),
                    stats_confirmed_download_count: row.get(6),
                    stats_anonymous_download_count: row.get(7)
                };
                indexes.push(index);
            }
        },
        Err(e) => {
            println!("Error getting indexes for ids, error {}", e);
        }
    };
    indexes
}

use ds::{IndexFile, IndexChunkItem};

pub fn insert_index(index: IndexFile) {
    let conn = establish_connection();

    let chunks = index.chunks;
    let mut chunk_ids_inserted = Vec::new();
    let initial_download_count = 0;
    let default_chunk_path = "NA".to_owned();
    for index_chunk_item in chunks.iter() {
        // Should remove index_id from chunk table as it creates cyclic dependency
        let index_id = 1;
        let dummy_chunk_size = 23;
        let initial_tags: Vec<i32> = Vec::new();
        match conn.query("INSERT INTO chunk(index_id, name, size,
                      creation_time, accessed_time, tags, stats_download_count)
                      VALUES( $1, $2, $3, $4, $5, $6, $7) RETURNING id",
                         &[&index_id,&index_chunk_item.name, &dummy_chunk_size,
                           &Utc::now(), &Utc::now(), &initial_tags,
                           &initial_download_count]) {
            Ok(rows) => {
                let id: i32 = rows.iter().next().unwrap().get(0);
                chunk_ids_inserted.push(id);
            },
            Err(e) => {
                println!("Could not insert chunks into chunk tables");
            }
        }
    }
    conn.execute("INSERT INTO index(name, path, chunks, creation_time, accessed_time,
                  stats_confirmed_download_count, stats_anonymous_download_count)
                  VALUES ($1, $2, $3, $4, $5, $6, $7) ", &[&index.name,
                                                       &default_chunk_path,
                                                       &chunk_ids_inserted,
                                                       &Utc::now(), &Utc::now(),
                                                       &initial_download_count,
                                                       &initial_download_count])
                 .expect("Could not insert into index table");
}
