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

fn main() {
    let default = rocket_cors::Cors::default();
    rocket::ignite()
        .mount("/", routes![root,
                            chunk_by_id,chunks_all,chunks_for_index_id,chunks_for_tag_id,
                            tag_by_id,tags_all,tags_for_chunk_id,tags_for_index_id,
                            indexes_by_id,indexes_all,
                            upload_index])
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

#[get("/chunks/<id>")]
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

#[get("/chunks")]
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

#[get("/chunks?<chunks_by_tag_query_params..>")]
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
fn upload_index(index_upload_params: Form<params::IndexUploadParams>, data: Data) {
    // TODO: Replace default_vendor_product_id with the one from auth
    let default_vendor_product_id = 1;
    let index_file = index_upload_params.name.to_owned();
    let path = "./test/".to_owned();
    let mut index_file_path = path.clone();
    index_file_path.push_str(&(index_file.to_owned()));

    match data.stream_to_file(index_file_path.clone()) {
        Ok(n) => {
            println!("Wrote {} bytes to file {}", n, index_file_path);
            let index_file_struct = ds::IndexFile::new(index_file, index_upload_params.version.to_owned());
            println!("Number of chunks read {}", index_file_struct.chunks.len());
            db::insert_index(index_file_struct,default_vendor_product_id);
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

