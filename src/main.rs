#![feature(proc_macro_hygiene, decl_macro)]
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
mod ds;
extern crate rocket_cors;
use rocket_cors::*;

fn main() {
    let default = rocket_cors::Cors::default();
    rocket::ignite()
        .mount("/", routes![root,chunks,tags,indexes,upload_index])
	      .attach(default) // Disable cors
        .launch();
}

#[get("/")]
fn root() -> &'static str {
    db::drop_tables();
    db::create_tables();
    db::load_seed_data();
    "Hello, world!"
}

#[get("/index/<id>/chunks")]
fn chunks(id: i32) -> Json<Vec<ds::ChunkItem>> {
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
    let chunk_array: Vec<ds::ChunkItem> = chunks.iter().map(|chunk| {
        let tag_ids_for_chunk = chunk.tags.as_ref().unwrap();
        let tags_filtered: Vec<&db::models::Tag> = tags.iter().filter(|tag| {
            tag_ids_for_chunk.iter().any(|x| x == &tag.id)
        }).collect();
        ds::ChunkItem::new(chunk.id, chunk.index_id, chunk.name.to_owned(),
                   chunk.size, chunk.creation_time.to_owned(),
                   chunk.accessed_time.to_owned(), tags_filtered,
                   chunk.stats_download_count)
    }).collect();
    Json(chunk_array)
}

#[get("/tag/<id>")]
fn tags(id: i32) -> Json<Vec<ds::TagItem>> {
    let tags = db::tags_for_ids(vec![id]);
    let tag_array: Vec<ds::TagItem> = tags.iter().map(|tag| {
        ds::TagItem::new(tag.id, tag.name.to_owned(), tag.creation_time, tag.accessed_time)
    }).collect();
    Json(tag_array)
}

#[get("/index/<id>")]
fn indexes(id: i32) -> Json<Vec<ds::IndexItem>> {
    let indexes = db::indexes_for_ids(vec![id]);

    let mut index_array: Vec<ds::IndexItem> = Vec::new();
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
        let chunk_array: Vec<ds::ChunkItem> = chunks.iter().map(|chunk| {
            let tag_ids_for_chunk = chunk.tags.as_ref().unwrap();
            let tags_filtered: Vec<&db::models::Tag> = tags.iter().filter(|tag| {
                tag_ids_for_chunk.iter().any(|x| x == &tag.id)
            }).collect();
            ds::ChunkItem::new(chunk.id, chunk.index_id, chunk.name.to_owned(),
                           chunk.size, chunk.creation_time.to_owned(),
                           chunk.accessed_time.to_owned(), tags_filtered,
                           chunk.stats_download_count)
        }).collect();
        let index = ds::IndexItem::new(index.id, index.name.to_owned(),
                                   index.path.to_owned(),chunk_array,
                                   index.creation_time, index.accessed_time,
                                   index.stats_confirmed_download_count,
                                   index.stats_anonymous_download_count);
        index_array.push(index);
    }
    Json(index_array)
}

//------------------------------------POST Requests----------------------------------//
use rocket::request::Form;
use rocket::data::Data;

#[derive(FromForm)]
struct IndexUploadParams {
    name: String,
    version: String
}

// If not multipart upload, can it be DOS'ed? (as it tries to read all data at once)
#[post("/upload/index?<index_upload_params..>", format="plain", data="<data>")]
fn upload_index(index_upload_params: Form<IndexUploadParams>, data: Data) {
    let index_file = index_upload_params.name.to_owned();
    let path = "./test/".to_owned();
    let mut index_file_path = path.clone();
    index_file_path.push_str(&(index_file.to_owned()));

    match data.stream_to_file(index_file_path.clone()) {
        Ok(n) => {
            println!("Wrote {} bytes to file {}", n, index_file_path);
            let index_file_struct = ds::IndexFile::new(index_file, index_upload_params.version.to_owned());
            println!("Number of chunks read {}", index_file_struct.chunks.len());
            db::insert_index(index_file_struct);
        },
        Err(e) => {
            println!("Error writing to file {}",e);
        }
    }
}

// // Get requests
// #[get("/indexes")]
// #[get("/chunk/<id>")]
// #[get("/chunks")]
// #[get("/tags")]

// // Post requests
// #[post("/index/type/<id>")]

