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
mod params;
use rocket::request::Form;
extern crate rocket_cors;
use rocket_cors::*;
use std::fs::DirBuilder;

fn main() {
    let default = rocket_cors::Cors::default();
    rocket::ignite()
        .mount("/", routes![root,
                            chunk_by_id,chunks_all,chunks_for_index_id,chunks_for_tag_id,
                            tag_by_id,tags_all,tags_for_chunk_id,tags_for_index_id,
                            tag_new,tag_update,tag_index,tag_index_remove,
                            indexes_by_id,indexes_all,
                            upload_index, upload_chunks,
                            add_index_download_count, add_chunk_download_count,
                            upload_blob, update_from_local_filesystem])
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

#[get("/chunks/<id>",rank=4)]
fn chunk_by_id(id: i32) -> Json<Vec<ds::ChunkItem>> {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;

    let chunks = db::chunks_for_ids(vec![id],default_vendor_product_id);
    let mut tag_ids: Vec<i32> = Vec::new();
    for chunk in chunks.iter() {
        let tags = chunk.tags.as_ref();
        tags.as_ref().unwrap().iter().for_each(|tag_id| {
            tag_ids.push(*tag_id);
        });
    }
    let tags = db::tags_for_ids(tag_ids,default_vendor_product_id);
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

#[get("/chunks",rank=3)]
fn chunks_all() -> Json<Vec<ds::ChunkItemRow>> {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    let chunks = db::chunks_all(default_vendor_product_id);
    println!("chunks len {}",chunks.len());
    let chunk_array: Vec<ds::ChunkItemRow> = chunks.iter().map(|chunk| {
        let tag_ids_for_chunk = chunk.tags.as_ref().unwrap();
        ds::ChunkItemRow::new(chunk.id, chunk.index_id, chunk.name.to_owned(),
                   chunk.size, chunk.creation_time.to_owned(),
                   chunk.accessed_time.to_owned(), tag_ids_for_chunk.to_owned(),
                   chunk.stats_download_count)
    }).collect();
    Json(chunk_array)
}

#[get("/chunks?<chunks_by_tag_query_params..>",rank=2)]
fn chunks_for_tag_id(chunks_by_tag_query_params: Form<params::ChunksByTagQueryParams>) -> Json<Vec<ds::ChunkItem>> {
    let id = chunks_by_tag_query_params.tag_id;
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    let chunks = db::chunks_for_tag_id(id, default_vendor_product_id);
    println!("chunks len {}",chunks.len());
    let mut tag_ids: Vec<i32> = Vec::new();
    for chunk in chunks.iter() {
        let tags = chunk.tags.as_ref();
        tags.as_ref().unwrap().iter().for_each(|tag_id| {
            tag_ids.push(*tag_id);
        });
    }
    let tags = db::tags_for_ids(tag_ids,default_vendor_product_id);
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

#[get("/chunks?<chunksByIndexQueryParams..>",rank=1)]
fn chunks_for_index_id(chunksByIndexQueryParams: Form<params::ChunksByIndexQueryParams>) -> Json<Vec<ds::ChunkItem>> {
    let id = chunksByIndexQueryParams.index_id;
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    let chunks = db::chunks_for_index_id(id, default_vendor_product_id);
    println!("chunks len {}",chunks.len());
    let mut tag_ids: Vec<i32> = Vec::new();
    for chunk in chunks.iter() {
        let tags = chunk.tags.as_ref();
        tags.as_ref().unwrap().iter().for_each(|tag_id| {
            tag_ids.push(*tag_id);
        });
    }
    let tags = db::tags_for_ids(tag_ids,default_vendor_product_id);
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

#[get("/tags/<id>")]
fn tag_by_id(id: i32) -> Json<Vec<ds::TagItem>> {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    let tags = db::tags_for_ids(vec![id],default_vendor_product_id);
    let tag_array: Vec<ds::TagItem> = tags.iter().map(|tag| {
        ds::TagItem::new(tag.id, tag.name.to_owned(), tag.creation_time, tag.accessed_time)
    }).collect();
    Json(tag_array)

    
}

#[get("/tags")]
fn tags_all() -> Json<Vec<ds::TagItem>> {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    let tags = db::tags_all(default_vendor_product_id);
    let tag_array: Vec<ds::TagItem> = tags.iter().map(|tag| {
        ds::TagItem::new(tag.id, tag.name.to_owned(), tag.creation_time, tag.accessed_time)
    }).collect();
    Json(tag_array)
}

#[get("/tags?<tags_by_chunk_query_params..>")]
fn tags_for_chunk_id(tags_by_chunk_query_params: Form<params::TagsByChunkQueryParams>) -> Json<Vec<ds::TagItem>> {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    let chunks = db::chunks_for_ids(vec![tags_by_chunk_query_params.chunk_id],default_vendor_product_id);
    let mut tag_ids: Vec<i32> = Vec::new();
    for chunk in chunks.iter() {
        let tags = chunk.tags.as_ref();
        tags.as_ref().unwrap().iter().for_each(|tag_id| {
            tag_ids.push(*tag_id);
        });
    }
    let tags = db::tags_for_ids(tag_ids,default_vendor_product_id);
    let tag_array: Vec<ds::TagItem> = tags.iter().map(|tag| {
        ds::TagItem::new(tag.id, tag.name.to_owned(), tag.creation_time, tag.accessed_time)
    }).collect();
    Json(tag_array)
}

#[get("/tags?<tags_by_index_query_params..>", rank=1)]
fn tags_for_index_id(tags_by_index_query_params: Form<params::TagsByIndexQueryParams>) -> Json<Vec<ds::TagItem>> {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;

   let indexes = db::indexes_for_ids(vec![tags_by_index_query_params.index_id],default_vendor_product_id);
    let mut tag_ids: Vec<i32> = Vec::new();
    for index in indexes.iter() {
        let mut chunk_ids: Vec<i32> = Vec::new();
        let chunks = index.chunks.as_ref();
        chunks.as_ref().unwrap().iter().for_each(|chunk_id| {
            chunk_ids.push(*chunk_id);
        });
        let chunks = db::chunks_for_ids(chunk_ids,default_vendor_product_id);
        for chunk in chunks.iter() {
            let tags = chunk.tags.as_ref();
            tags.as_ref().unwrap().iter().for_each(|tag_id| {
                tag_ids.push(*tag_id);
            });
        }
    }
    let tags = db::tags_for_ids(tag_ids,default_vendor_product_id);
    let tag_array: Vec<ds::TagItem> = tags.iter().map(|tag| {
        ds::TagItem::new(tag.id, tag.name.to_owned(), tag.creation_time, tag.accessed_time)
    }).collect();
    Json(tag_array)
}

#[put("/tags/new?<tag_new_query_params..>")]
fn tag_new(tag_new_query_params: Form<params::TagNewQueryParams>) -> Json<ds::TagItem> {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    Json(db::tag_new(tag_new_query_params.name.to_owned(), default_vendor_product_id))
}

#[put("/tags/update?<tag_update_query_params..>")]
fn tag_update(tag_update_query_params: Form<params::TagUpdateQueryParams>) {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    db::tag_update(tag_update_query_params.id, tag_update_query_params.name.to_owned(), default_vendor_product_id)
}

#[put("/tags/add?<tag_index_query_params..>", rank=1)]
fn tag_index(tag_index_query_params: Form<params::TagIndexQueryParams>) {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    db::tag_index(tag_index_query_params.id, tag_index_query_params.index_id, default_vendor_product_id)
}

#[put("/tags/remove?<tag_index_query_params..>", rank=2)]
fn tag_index_remove(tag_index_query_params: Form<params::TagIndexQueryParams>) {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    db::tag_index_remove(tag_index_query_params.id, tag_index_query_params.index_id, default_vendor_product_id)
}

#[get("/indexes/<id>")]
fn indexes_by_id(id: i32) -> Json<Vec<ds::IndexItem>> {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    let indexes = db::indexes_for_ids(vec![id],default_vendor_product_id);

    let mut index_array: Vec<ds::IndexItem> = Vec::new();
    for index in indexes.iter() {
        let mut chunk_ids: Vec<i32> = Vec::new();
        let chunks = index.chunks.as_ref();
        chunks.as_ref().unwrap().iter().for_each(|chunk_id| {
            chunk_ids.push(*chunk_id);
        });
        let chunks = db::chunks_for_ids(chunk_ids,default_vendor_product_id);
        let mut tag_ids: Vec<i32> = Vec::new();
        for chunk in chunks.iter() {
            let tags = chunk.tags.as_ref();
            tags.as_ref().unwrap().iter().for_each(|tag_id| {
                tag_ids.push(*tag_id);
            });
        }
        let tags = db::tags_for_ids(tag_ids,default_vendor_product_id);
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
        let index = ds::IndexItem::new(index.id, index.name.to_owned(),chunk_array,
                                   index.creation_time, index.accessed_time,
                                   index.stats_confirmed_download_count,
                                   index.stats_anonymous_download_count);
        index_array.push(index);
    }
    Json(index_array)
}

#[get("/indexes")]
fn indexes_all() -> Json<Vec<ds::IndexItemRow>> {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    let indexes = db::indexes_all(default_vendor_product_id);
    let mut index_array: Vec<ds::IndexItemRow> = Vec::new();
    for index in indexes.iter() {
        let index_row = ds::IndexItemRow::new(index.id, index.name.to_owned(),
                                   index.creation_time, index.accessed_time,
                                   index.stats_confirmed_download_count,
                                   index.stats_anonymous_download_count);
        index_array.push(index_row);
    }
    Json(index_array)
}
//------------------------------------POST Requests----------------------------------//
use rocket::data::Data;

// If not multipart upload, can it be DOS'ed? (as it tries to read all data at once)
#[post("/upload/index?<index_upload_params..>", format="plain", data="<data>")]
fn upload_index(index_upload_params: Form<params::IndexUploadParams>, data: Data) -> Json<Vec<i32>>{
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    let index_file = index_upload_params.name.to_owned();
    match db::vendor_product_for_id(default_vendor_product_id) {
        Some(vp) => {
            let mut index_file_path = "".to_owned();
            let vendor_name = vp.vendor_name;
            let product_name = vp.product_name;
            index_file_path.push_str(&(vendor_name));
            index_file_path.push_str("/");
            index_file_path.push_str(&(product_name));
            let mut full_index_file_path = index_file_path.clone();
            full_index_file_path.push_str("/");
            full_index_file_path.push_str(&(index_file.to_owned()));
            
            println!("dir to store index {}", index_file_path);
            DirBuilder::new()
                .recursive(true)
                .create(index_file_path.clone());
            match data.stream_to_file(full_index_file_path.clone()) {
                Ok(n) => {
                    println!("Wrote {} bytes to file {}", n, index_file_path);
                    let index_file_struct = ds::IndexFile::new(index_file_path, index_file, index_upload_params.version.to_owned());
                    println!("Number of chunks read {}", index_file_struct.chunks.len());
                    match db::insert_index(index_file_struct,default_vendor_product_id) {
                        Some(index_id) => Json(vec![index_id]),
                        None => Json(vec![])
                    }
                },
                Err(e) => {
                    println!("Error writing to file {}",e);
                    Json(vec![])
                }
            }
        },
        None => {
            println!("Could not upload index file");
            Json(vec![])
        }
    }
}

#[post("/upload/chunk?<chunk_upload_params..>", format="plain", data="<data>")]
fn upload_chunks(chunk_upload_params: Form<params::ChunkUploadParams>, data: Data) {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    let mut chunk_file = chunk_upload_params.name.clone();
    match db::vendor_product_for_id(default_vendor_product_id) {
        Some(vp) => {
            let mut chunk_file_path = "".to_owned();
            let vendor_name = vp.vendor_name;
            let product_name = vp.product_name;
            chunk_file_path.push_str(&(vendor_name));
            chunk_file_path.push_str("/");
            chunk_file_path.push_str(&(product_name));
            chunk_file_path.push_str("/");
            chunk_file_path.push_str("store");

            let mut full_chunk_file_path = chunk_file_path.clone();
            full_chunk_file_path.push_str("/");
            full_chunk_file_path.push_str(&(chunk_file.to_owned()));
            
            println!("chunk {} added", full_chunk_file_path);
            DirBuilder::new()
                .recursive(true)
                .create(chunk_file_path.clone());
            match data.stream_to_file(full_chunk_file_path.clone()) {
                Ok(n) => {
                    println!("Wrote {} bytes to file {}", n, chunk_file_path);
                    chunk_file.truncate(64);
                    db::update_chunk_file_exists(chunk_upload_params.index_id, chunk_file, default_vendor_product_id);
                },
                Err(e) => {
                    println!("Error writing to file {}",e);
                }
            }
        },
        None => {
            println!("Could not upload chunk file")
        }
    }
}

use std::process::Command;

#[post("/upload/blob?<blob_upload_params..>", format="plain", data="<data>")]
fn upload_blob(blob_upload_params: Form<params::BlobUploadParams>, data: Data) {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    let blob_file = blob_upload_params.blob_name.to_owned();
    let index_file = blob_upload_params.index_name.to_owned();
    match db::vendor_product_for_id(default_vendor_product_id) {
        Some(vp) => {
            let mut blob_file_path = "".to_owned();
            let vendor_name = vp.vendor_name;
            let product_name = vp.product_name;
            blob_file_path.push_str(&(vendor_name));
            blob_file_path.push_str("/");
            blob_file_path.push_str(&(product_name));
            let mut full_blob_file_path = blob_file_path.clone();
            full_blob_file_path.push_str("/");
            full_blob_file_path.push_str(&(blob_file.to_owned()));

            let mut full_chunk_store_path = blob_file_path.clone();
            full_chunk_store_path.push_str("/");
            full_chunk_store_path.push_str("store");

            let mut full_index_file_path = blob_file_path.clone();
            full_index_file_path.push_str("/");
            full_index_file_path.push_str(&(index_file.to_owned()));
            
            DirBuilder::new()
                .recursive(true)
                .create(blob_file_path.clone());
            DirBuilder::new()
                .recursive(true)
                .create(full_chunk_store_path.clone());
            match data.stream_to_file(full_blob_file_path.clone()) {
                Ok(n) => {
                    Command::new("binaries/desync")
                        .args(&["make", &full_index_file_path, "-s", &full_chunk_store_path, &full_blob_file_path])
                        .output()
                        .expect("Could not list chunks");
                    
                    let out = Command::new("binaries/desync")
                        .args(&["list-chunks", &full_index_file_path])
                        .output()
                        .expect("Could not list chunks");
 
                    if out.status.success() {
                        let all_chunks = String::from_utf8_lossy(&out.stdout).to_string();
                        let mut index_chunk_items = Vec::new();
                        for chunk_name in all_chunks.lines() {
                            let index_chunk_item = ds::IndexChunkItem {
                                name: chunk_name.to_owned(),
                                // TODO: Figure out actual size of chunk
                                size: 0
                            };
                            index_chunk_items.push(index_chunk_item);
                        }
                        let index_file = ds::IndexFile {
                            name: index_file.to_owned(),
                            //TODO: Check if this is necessary if tag already exists
                            version: "".to_owned(),
                            path: full_index_file_path.to_owned(),
                            chunks: index_chunk_items
                        };
                        match db::insert_index(index_file,default_vendor_product_id) {
                            Some(index_id) => Json(vec![index_id]),
                            None => Json(vec![])
                        };
                    } else {
                        String::from("Listing chunks not successful");
                    }
                },
                Err(e) => {
                    println!("Error writing blob to file {}",e);
                }
            }
        },
        None => {
            println!("Could not upload blob file");
        }
    }
}

#[post("/stats/download?<stats_index_download..>", rank=2)]
fn add_index_download_count(stats_index_download: Form<params::StatsIndexDownload>) {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    db::add_index_download_count(stats_index_download.index_id, stats_index_download.index_name.to_owned(), 
                                stats_index_download.confirmed_count, 
                                stats_index_download.anonymous_count, default_vendor_product_id)
}

#[post("/stats/download?<stats_chunk_download..>", rank=1)]
fn add_chunk_download_count(stats_chunk_download: Form<params::StatsChunkDownload>) {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    db::add_chunk_download_count(stats_chunk_download.chunk_id, stats_chunk_download.chunk_name.to_owned(), 
                                stats_chunk_download.confirmed_count, default_vendor_product_id)
}

// Local indexes and store 
use dotenv::dotenv;
use std::env;
use std::fs;
use std::path::Path;

extern crate regex;
use regex::Regex;

pub struct LocalFile {
    path: String,
    file_name: String
}

#[get("/update/filesystem")]
fn update_from_local_filesystem() {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    
    dotenv().ok();
    let local_index_file_path = match env::var("LOCAL_INDEXES_PATH") {
        Ok(val) => val.clone(),
        Err(e) => "".to_owned()
    };
    let local_chunks_file_path = match env::var("LOCAL_CHUNKS_PATH") {
        Ok(val) => val.clone(),
        Err(e) => "".to_owned()
    };
    
    let index_files = get_files(local_index_file_path, vec!["^*.caidx$".to_owned(), "^*.caibx$".to_owned()]);
    let chunk_files = get_files(local_chunks_file_path, vec!["^*.cacnk$".to_owned()]);
    for local_index_file in index_files.iter() {
        let out = Command::new("binaries/desync")
            .args(&["list-chunks", &local_index_file.path])
            .output()
            .expect("Could not list chunks");
        let index_file_name = local_index_file.file_name.to_owned();
        if out.status.success() {                
            let all_chunks = String::from_utf8_lossy(&out.stdout).to_string();
            let mut index_chunk_items = Vec::new();
            for chunk_name in all_chunks.lines() {
                let mut chunk_name_with_ext = chunk_name.to_owned();
                chunk_name_with_ext.push_str(".cacnk");
                for local_chunk_file in chunk_files.iter() {
                    //TODO: If the chunk file doesn't exist on filesystem
                    if local_chunk_file.file_name == chunk_name_with_ext {
                        let file_size = fs::metadata(local_chunk_file.path.to_owned()).unwrap().len();
                        let index_chunk_item = ds::IndexChunkItem {
                            name: chunk_name.to_owned(),
                            // TODO: Figure out actual size of chunk
                            size: file_size as i64
                        };
                        index_chunk_items.push(index_chunk_item);
                        break;
                    }
                }
            }
            let index_file = ds::IndexFile {
                name: index_file_name.to_owned(),
                //TODO: Check if this is necessary if tag already exists
                version: "".to_owned(),
                path: local_index_file.path.to_owned(),
                chunks: index_chunk_items.clone()
            };
            match db::insert_index(index_file,default_vendor_product_id) {
                Some(index_id) => {
                    for index_chunk_item in index_chunk_items.iter() {
                        db::update_chunk_file_exists(index_id, 
                            index_chunk_item.name.to_owned(), default_vendor_product_id);
                    }
                },
                None => ()
            };
        } else {
            panic!("Could not list chunks from the index files");
        }
    }
}

fn get_files(path: String, formats: Vec<String>) -> Vec<LocalFile>{
    let mut files: Vec<LocalFile> = Vec::new();
    let mut rexps: Vec<Regex> = Vec::new();

    for format in formats {
        rexps.push(Regex::new(&format).unwrap());
    }

    read_dir_recursive(path.to_owned(), &mut files, rexps.clone());
    // for file in files.iter() {
    //     println!("File-> {}", file.file_name.to_owned());
    // }
    files
}

fn read_dir_recursive(path: String, files: &mut Vec<LocalFile>, rexps: Vec<Regex>) {
    if Path::new(&path).is_dir() {
        for entry in fs::read_dir(&path).unwrap() {
            let dir = entry.unwrap();
            read_dir_recursive(dir.path().into_os_string().into_string().unwrap().to_owned(), files, rexps.clone());
        }
    } else {
        for rexp in rexps.iter() {
            if rexp.is_match(&path) {
                let localFile = LocalFile {
                    path: path.to_owned(),
                    file_name: Path::new(&path).file_name().unwrap().to_str().unwrap().to_owned()
                };
                files.push(localFile);
            }
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

