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
extern crate diesel;
extern crate dotenv;
extern crate chrono;


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
pub struct Chunk {
    id: i32,
    index_id: i32,
    name: String,
    size: i32,
    creation_time: String,
    accessed_time: String,
    tags: Vec<TagItem>,
    stats_download_count: i32
}

impl Chunk {
    pub fn new(id: i32, index_id: i32, name: String, size: i32, creation_time: String, accessed_time: String, tags: Option<&Vec<i32>>, download_count: i32) -> Chunk {
        let mut local_tags = Vec::new();
        tags.as_ref().unwrap().iter().for_each(|tag| {
            local_tags.push(TagItem::new(*tag, "".to_string(), chrono::Utc::now(), chrono::Utc::now()))
        });
        Chunk {
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

fn main() {
    rocket::ignite().mount("/", routes![root,chunks]).launch();
}

#[get("/")]
fn root() -> &'static str {
    "Hello, world!"
}

#[get("/index/<id>/chunks")]
fn chunks(id: i32) -> Json<Vec<Chunk>> {
    let chunks = db::chunks_from_index_id(id);
    let mut tag_ids: Vec<i32> = Vec::new();
    for chunk in chunks.iter() {
        let tags = chunk.tags.as_ref();
        tags.as_ref().unwrap().iter().for_each(|tag_id| {
            tag_ids.push(*tag_id);
        });
    }
    let tags = db::tags_from_ids(tag_ids);
    let chunk_array: Vec<Chunk> = chunks.iter().map(|chunk| {
        let tags = chunk.tags.as_ref();
        Chunk::new(chunk.id, chunk.index_id, chunk.name.to_owned(),
                   chunk.size, chunk.creation_time.to_owned(),
                   chunk.accessed_time.to_owned(), tags,
                   chunk.stats_download_count)
    }).collect();
    Json(chunk_array)
}

// // Get requests
// #[get("/indexes")]
// #[get("/chunk/<id>")]
// #[get("/chunks")]
// #[get("/tags")]
// #[get("/tag/<id>")]

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
