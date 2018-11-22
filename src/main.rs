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

#[derive(Serialize, Deserialize)]
pub struct TagItem {
    id: i32,
    name: String
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
            local_tags.push(TagItem::new(*tag, "".to_string()))
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
    pub fn new(id: i32, name: String) -> TagItem {
        TagItem {
            id: id,
            name: name
        }
    }
}

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::json::{Json};
mod db;

fn main() {
    rocket::ignite().mount("/", routes![root,chunks]).launch();
}

#[get("/")]
fn root() -> &'static str {
    "Hello, world!"
}

#[get("/chunks/index/<id>")]
fn chunks(id: i32) -> Json<Vec<Chunk>> {
    let chunks = db::chunks_from_index_id(id);
    let chunk_array: Vec<Chunk> = chunks.iter().map(|chunk| {
        let tags = chunk.tags.as_ref();
        Chunk::new(chunk.id, chunk.index_id, chunk.name.to_owned(),
                   chunk.size, chunk.creation_time.to_owned(),
                   chunk.accessed_time.to_owned(), tags,
                   chunk.stats_download_count)
    }).collect();
    Json(chunk_array)
}
