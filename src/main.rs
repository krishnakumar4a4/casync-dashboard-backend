#![feature(proc_macro_hygiene, decl_macro)]
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Write;

extern crate serde;
//#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate dotenv;
extern crate chrono;

extern crate postgres;

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::json::{Json};
mod db;

use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct TagItem {
    id: i32,
    name: String,
    #[serde(with = "my_date_format")]
    creation_time: chrono::DateTime<Utc>,
    #[serde(with = "my_date_format")]
    accessed_time: chrono::DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct ChunkItem {
    id: i32,
    index_id: i32,
    name: String,
    size: i32,
    #[serde(with = "my_date_format")]
    creation_time: chrono::DateTime<Utc>,
    #[serde(with = "my_date_format")]
    accessed_time: chrono::DateTime<Utc>,
    tags: Vec<TagItem>,
    stats_download_count: i32
}

#[derive(Serialize, Deserialize)]
pub struct IndexItem {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub chunks: Vec<ChunkItem>,
    #[serde(with = "my_date_format")]
    pub creation_time: chrono::DateTime<Utc>,
    #[serde(with = "my_date_format")]
    pub accessed_time: chrono::DateTime<Utc>,
    pub stats_confirmed_download_count: i32,
    pub stats_anonymous_download_count: i32
}

impl ChunkItem {
    pub fn new(id: i32, index_id: i32, name: String, size: i32, creation_time: chrono::DateTime<Utc>, accessed_time: chrono::DateTime<Utc>, tags: Vec<&db::models::Tag>, download_count: i32) -> ChunkItem {
        let mut local_tags = Vec::new();
        for tag in tags.iter() {
            local_tags.push(TagItem::new(tag.id, tag.name.to_owned(),
                                         tag.creation_time,
                                         tag.accessed_time));
        }
        ChunkItem {
            id: id,
            index_id: index_id,
            name: name,
            size: size,
            creation_time: creation_time,
            accessed_time: accessed_time,
            tags: local_tags,
            stats_download_count: download_count
        }
    }
}

impl TagItem {
    pub fn new(id: i32, name: String, creation_time: chrono::DateTime<Utc>, accessed_time: chrono::DateTime<Utc>) -> TagItem {
        TagItem {
            id: id,
            name: name,
            creation_time: creation_time,
            accessed_time: accessed_time
        }
    }
}

impl IndexItem {
    pub fn new(id: i32, name: String, path: String,
               chunks: Vec<ChunkItem>,
               creation_time: chrono::DateTime<Utc>,
               accessed_time: chrono::DateTime<Utc>,
               stats_confirmed_download_count: i32,
               stats_anonymous_download_count: i32) -> IndexItem {
        IndexItem {
            id: id,
            name: name,
            path: path,
            chunks: chunks,
            creation_time: creation_time,
            accessed_time: accessed_time,
            stats_confirmed_download_count: stats_confirmed_download_count,
            stats_anonymous_download_count: stats_anonymous_download_count
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![root,chunks,tags,indexes]).launch();
}

#[get("/")]
fn root() -> &'static str {
    db::drop_tables();
    db::create_tables();
    db::load_seed_data();
    "Hello, world!"
}

#[get("/index/<id>/chunks")]
fn chunks(id: i32) -> Json<Vec<ChunkItem>> {
    let chunks = db::chunks_for_index_id(id);
    println!("chunks len {}",chunks.len());
    let mut tag_ids: Vec<i32> = Vec::new();
    for chunk in chunks.iter() {
        let tags = chunk.tags.as_ref();
        tags.as_ref().unwrap().iter().for_each(|tag_id| {
            tag_ids.push(*tag_id);
        });
    }
    let tags = db::tags_for_ids(tag_ids);
    let chunk_array: Vec<ChunkItem> = chunks.iter().map(|chunk| {
        let tag_ids_for_chunk = chunk.tags.as_ref().unwrap();
        let tags_filtered: Vec<&db::models::Tag> = tags.iter().filter(|tag| {
            tag_ids_for_chunk.iter().any(|x| x == &tag.id)
        }).collect();
        ChunkItem::new(chunk.id, chunk.index_id, chunk.name.to_owned(),
                   chunk.size, chunk.creation_time.to_owned(),
                   chunk.accessed_time.to_owned(), tags_filtered,
                   chunk.stats_download_count)
    }).collect();
    Json(chunk_array)
}

#[get("/tag/<id>")]
fn tags(id: i32) -> Json<Vec<TagItem>> {
    let tags = db::tags_for_ids(vec![id]);
    let tag_array: Vec<TagItem> = tags.iter().map(|tag| {
        TagItem::new(tag.id, tag.name.to_owned(), tag.creation_time, tag.accessed_time)
    }).collect();
    Json(tag_array)
}

#[get("/index/<id>")]
fn indexes(id: i32) -> Json<Vec<IndexItem>> {
    let indexes = db::indexes_for_ids(vec![id]);

    let mut index_array: Vec<IndexItem> = Vec::new();
    for index in indexes.iter() {
        let mut chunk_ids: Vec<i32> = Vec::new();
        for index in indexes.iter() {
            let chunks = index.chunks.as_ref();
            chunks.as_ref().unwrap().iter().for_each(|chunk_id| {
                chunk_ids.push(*chunk_id);
            });
        }
        let chunks = db::chunks_for_ids(chunk_ids);
        let mut tag_ids: Vec<i32> = Vec::new();
        for chunk in chunks.iter() {
            let tags = chunk.tags.as_ref();
            tags.as_ref().unwrap().iter().for_each(|tag_id| {
                tag_ids.push(*tag_id);
            });
        }
        let tags = db::tags_for_ids(tag_ids);
        let chunk_array: Vec<ChunkItem> = chunks.iter().map(|chunk| {
            let tag_ids_for_chunk = chunk.tags.as_ref().unwrap();
            let tags_filtered: Vec<&db::models::Tag> = tags.iter().filter(|tag| {
                tag_ids_for_chunk.iter().any(|x| x == &tag.id)
            }).collect();
            ChunkItem::new(chunk.id, chunk.index_id, chunk.name.to_owned(),
                           chunk.size, chunk.creation_time.to_owned(),
                           chunk.accessed_time.to_owned(), tags_filtered,
                           chunk.stats_download_count)
        }).collect();
        let index = IndexItem::new(index.id, index.name.to_owned(),
                                   index.path.to_owned(),chunk_array,
                                   index.creation_time, index.accessed_time,
                                   index.stats_confirmed_download_count,
                                   index.stats_anonymous_download_count);
        index_array.push(index);
    }
    Json(index_array)
}

// // Get requests
// #[get("/indexes")]
// #[get("/chunk/<id>")]
// #[get("/chunks")]
// #[get("/tags")]

// // Post requests
// #[post("/index/type/<id>")]

// Ref: https://serde.rs/custom-date-format.html
mod my_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
