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
    conn.execute("CREATE TABLE IF NOT EXISTS vendor (
        id SERIAL PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        creation_time timestamp with time zone NOT NULL,
        accessed_time timestamp with time zone NOT NULL 
    )",&[]).expect("Table vendor doesn't exist and could not create it");
        conn.execute("CREATE TABLE IF NOT EXISTS product (
        id SERIAL PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        creation_time timestamp with time zone NOT NULL,
        accessed_time timestamp with time zone NOT NULL 
    )",&[]).expect("Table product doesn't exist and could not create it");
        conn.execute("CREATE TABLE IF NOT EXISTS vendor_product (
        id SERIAL PRIMARY KEY NOT NULL,
        vendor_id INT NOT NULL,
        product_id INT NOT NULL,
        creation_time timestamp with time zone NOT NULL,
        accessed_time timestamp with time zone NOT NULL 
    )",&[]).expect("Table vendor_product doesn't exist and could not create it");
    conn.execute("CREATE TABLE IF NOT EXISTS chunk (
       id SERIAL PRIMARY KEY NOT NULL,
       index_id INT NOT NULL,
       name TEXT NOT NULL,
       size BIGINT NOT NULL,
       creation_time timestamp with time zone NOT NULL,
       accessed_time timestamp with time zone NOT NULL,
       tags integer ARRAY,
       stats_download_count INT NOT NULL,
       vendor_product_id INT NOT NULL,
       file_exists boolean default false NOT NULL)", &[]).expect("Table chunk doesn't exist and could not create it");
    conn.execute("CREATE TABLE IF NOT EXISTS tag (
       id SERIAL PRIMARY KEY NOT NULL,
       name TEXT NOT NULL,
       creation_time timestamp with time zone NOT NULL,
       accessed_time timestamp with time zone NOT NULL,
       vendor_product_id INT NOT NULL)", &[]).expect("Table tag doesn't exist and could not create it");
    conn.execute("CREATE TABLE IF NOT EXISTS index (
       id SERIAL PRIMARY KEY NOT NULL,
       name TEXT NOT NULL,
       chunks integer ARRAY,
       creation_time timestamp with time zone NOT NULL,
       accessed_time timestamp with time zone NOT NULL,
       stats_confirmed_download_count INT NOT NULL,
       stats_anonymous_download_count INT NOT NULL,
       vendor_product_id INT NOT NULL,
       tag_id INT
       )", &[]).expect("Table index doesn't exist and could not create it");
}

pub fn drop_tables() {
    let conn = establish_connection();
    conn.execute("DROP TABLE IF EXISTS vendor",&[]).expect("Could not drop table vendor");
    conn.execute("DROP TABLE IF EXISTS product",&[]).expect("Could not drop table product");
    conn.execute("DROP TABLE IF EXISTS vendor_product",&[]).expect("Could not drop table vendor_product");
    conn.execute("DROP TABLE IF EXISTS chunk",&[]).expect("Could not drop table chunk");
    conn.execute("DROP TABLE IF EXISTS tag",&[]).expect("Could not drop table tag");
    conn.execute("DROP TABLE IF EXISTS index", &[]).expect("Could not drop table index");
}

pub fn load_seed_data() {
    let conn = establish_connection();
    let default_vendor_product_id = 1;
    // Insert default values to vendor and product tables
    let defaultVendor = models::Vendor {
        id: 1,
        name: "default".to_owned(),
        creation_time: Utc::now(),
        accessed_time: Utc::now()
    };

    let defaultProduct = models::Product {
        id: 1,
        name: "default".to_owned(),
        creation_time: Utc::now(),
        accessed_time: Utc::now()
    };

    let defaultVendorProduct = models::VendorProduct {
        id: 1,
        vendor_id: 1,
        vendor_name: "default".to_string(),
        product_id: 1,
        product_name: "default".to_string(),
        creation_time: Utc::now(),
        accessed_time: Utc::now()
    };

    conn.execute("INSERT INTO vendor(id, name, creation_time, accessed_time) VALUES($1,$2,$3,$4);", 
                &[&defaultVendor.id, &defaultVendor.name, &defaultVendor.creation_time, &defaultVendor.accessed_time])
                .expect("Could not insert default vendor into vendor table");
    conn.execute("INSERT INTO product(id, name, creation_time, accessed_time) VALUES($1,$2,$3,$4);", 
                &[&defaultProduct.id, &defaultProduct.name, &defaultProduct.creation_time, &defaultProduct.accessed_time])
                .expect("Could not insert default product into product table");
    conn.execute("INSERT INTO vendor_product(id, vendor_id, product_id, creation_time, accessed_time) VALUES($1,$2,$3,$4,$5);", 
                &[&defaultVendorProduct.id, &defaultVendorProduct.vendor_id, &defaultVendorProduct.product_id, &defaultVendorProduct.creation_time, &defaultVendorProduct.accessed_time])
                .expect("Could not insert default vendor and product into vendor_product table");

    // Insert chunks to chunk table
    let chunk1 = models::Chunk{
        id: 1,
        index_id: 1,
        name: "Chunk1".to_owned(),
        size: 11,
        creation_time: Utc::now(),
        accessed_time: Utc::now(),
        tags: Some(vec![1,2]),
        stats_download_count: 0,
        vendor_product_id: 1
    };
    let chunk2 = models::Chunk{
        id: 2,
        index_id: 1,
        name: "Chunk2".to_owned(),
        size: 12,
        creation_time: Utc::now(),
        accessed_time: Utc::now(),
        tags: Some(vec![1]),
        stats_download_count: 0,
        vendor_product_id: 1
    };
    conn.execute("insert into chunk(index_id, name, size,
                   creation_time, accessed_time, tags, stats_download_count, vendor_product_id)
                   values($1,$2,$3,$4,$5,$6,$7,$8);",
                 &[&chunk1.index_id,&chunk1.name,&chunk1.size,
                   &chunk1.creation_time, &chunk1.accessed_time, &chunk1.tags,
                   &chunk1.stats_download_count, &chunk1.vendor_product_id]).expect("Could not insert seed data into chunk table");

    conn.execute("insert into chunk(index_id, name, size,
                   creation_time, accessed_time, tags, stats_download_count, vendor_product_id)
                   values($1,$2,$3,$4,$5,$6,$7,$8);",
                 &[&chunk2.index_id,&chunk2.name,&chunk2.size,
                   &chunk2.creation_time, &chunk2.accessed_time, &chunk2.tags,
                   &chunk2.stats_download_count, &chunk2.vendor_product_id]).expect("Could not insert seed data into chunk table");

    // Insert tags into tag table
    let tag1 = models::Tag {
        id: 1,
        name: "rel1".to_owned(),
        creation_time: Utc::now(),
        accessed_time: Utc::now(),
        vendor_product_id: 1
    };
    conn.execute("INSERT INTO tag VALUES ($1, $2, $3, $4, $5)",
                 &[&tag1.id, &tag1.name, &tag1.creation_time, &tag1.accessed_time, &tag1.vendor_product_id])
        .expect("Could not insert seed data into tag table");


    // Insert tags into tag table
    let tag2 = models::Tag {
        id: 2,
        name: "rel2".to_owned(),
        creation_time: Utc::now(),
        accessed_time: Utc::now(),
        vendor_product_id: 1
    };
    conn.execute("INSERT INTO tag VALUES ($1, $2, $3, $4, $5)",
                 &[&tag2.id, &tag2.name, &tag2.creation_time, &tag2.accessed_time, &tag2.vendor_product_id])
        .expect("Could not insert seed data into tag table");

    //Insert indexes into index table
    let index1 = models::Index{
        id: 1,
        name: "index1.caibx".to_owned(),
        chunks: Some(vec![1,2]),
        creation_time: Utc::now(),
        accessed_time: Utc::now(),
        stats_confirmed_download_count: 0,
        stats_anonymous_download_count: 0,
        vendor_product_id: 1
    };
    conn.execute("INSERT INTO index(name, chunks, creation_time, accessed_time,
                  stats_confirmed_download_count, stats_anonymous_download_count, vendor_product_id)
                  VALUES ($1, $2, $3, $4, $5, $6, $7)",
                 &[&index1.name, &index1.chunks, &index1.creation_time,
                   &index1.accessed_time, &index1.stats_confirmed_download_count,
                   &index1.stats_anonymous_download_count, &index1.vendor_product_id])
        .expect("Could not insert seed data into index table");

    let index2 = models::Index{
        id: 2,
        name: "index2.caibx".to_owned(),
        chunks: Some(vec![1]),
        creation_time: Utc::now(),
        accessed_time: Utc::now(),
        stats_confirmed_download_count: 0,
        stats_anonymous_download_count: 0,
        vendor_product_id: 1
    };
    conn.execute("INSERT INTO index(name, chunks, creation_time, accessed_time,
                  stats_confirmed_download_count, stats_anonymous_download_count, vendor_product_id)
                  VALUES ($1, $2, $3, $4, $5, $6, $7)",
                 &[&index2.name, &index2.chunks, &index2.creation_time,
                   &index2.accessed_time, &index2.stats_confirmed_download_count,
                   &index2.stats_anonymous_download_count, &index2.vendor_product_id])
        .expect("Could not insert seed data into index table");
}

pub fn vendor_product_for_id(vendor_product_id: i32) -> Option<models::VendorProduct> {
    let conn = establish_connection();
    match conn.query("select vp.id, v.id, v.name, p.id, p.name, vp.creation_time, vp.accessed_time
                        from vendor_product vp 
                        inner join vendor v on v.id = vp.vendor_id 
                        inner join product p on p.id = vp.product_id where vp.id=$1", &[&vendor_product_id]) {
        Ok(rows) => {
            let row = rows.get(0);
            Some(models::VendorProduct {
                id: row.get(0),
                vendor_id: row.get(1),
                vendor_name: row.get(2),
                product_id: row.get(3),
                product_name: row.get(4),
                creation_time: row.get(5),
                accessed_time: row.get(6)
            })
        }
        Err(e) => {
            println!("Could not get vendor product for id {}, error {}", vendor_product_id, e);
            None
        }
    }
}

pub fn chunks_all(vendor_product_id: i32) -> Vec<models::Chunk> {
    let conn = establish_connection();
    let mut chunks: Vec<models::Chunk> = Vec::new();

    match conn.query("SELECT id,index_id,name,size,creation_time,accessed_time,tags,stats_download_count,
                        vendor_product_id FROM chunk where vendor_product_id = $1", &[&vendor_product_id]) {
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
                    stats_download_count: row.get(7),
                    vendor_product_id: row.get(8)
                };
                chunks.push(chunk);
            };
        },
        Err(e) => {
            println!("Error getting all chunks ");
        }
    };
    chunks
}

pub fn chunks_for_index_id(index_id: i32, vendor_product_id: i32) -> Vec<models::Chunk> {
    let conn = establish_connection();
    let mut chunks: Vec<models::Chunk> = Vec::new();

    match conn.query("SELECT id,index_id,name,size,creation_time,accessed_time,tags,stats_download_count,vendor_product_id FROM chunk where index_id = $1 AND vendor_product_id = $2", &[&index_id, &vendor_product_id]) {
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
                    stats_download_count: row.get(7),
                    vendor_product_id: row.get(8)
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

pub fn chunks_for_tag_id(tag_id: i32, vendor_product_id: i32) -> Vec<models::Chunk> {
    let conn = establish_connection();
    let mut chunks: Vec<models::Chunk> = Vec::new();

    match conn.query("SELECT id,index_id,name,size,creation_time,accessed_time,tags,stats_download_count,vendor_product_id FROM chunk where tags @> $1 AND vendor_product_id = $2", &[&vec![tag_id], &vendor_product_id]) {
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
                    stats_download_count: row.get(7),
                    vendor_product_id: row.get(8)
                };
                chunks.push(chunk);
            };
        },
        Err(e) => {
            println!("Error getting chunks for tag_id {}", tag_id);
        }
    };
    chunks
}

pub fn chunks_for_ids(ids: Vec<i32>, vendor_product_id: i32) -> Vec<models::Chunk> {
    let conn = establish_connection();
    let mut chunks: Vec<models::Chunk> = Vec::new();

    match conn.query("SELECT id,index_id,name,size,creation_time,accessed_time,tags,stats_download_count,vendor_product_id FROM chunk where id = ANY($1) AND vendor_product_id = $2", &[&ids, &vendor_product_id]) {
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
                    stats_download_count: row.get(7),
                    vendor_product_id: row.get(8)
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

pub fn tags_all(vendor_product_id: i32) -> Vec<models::Tag> {
    let conn = establish_connection();
    let mut tags: Vec<models::Tag> = Vec::new();

    match conn.query("SELECT id, name, creation_time, accessed_time, vendor_product_id FROM tag WHERE vendor_product_id = $1", &[&vendor_product_id]) {
        Ok(rows) => {
            for row in rows.iter() {
                let tag = models::Tag {
                    id: row.get(0),
                    name: row.get(1),
                    creation_time: row.get(2),
                    accessed_time: row.get(3),
                    vendor_product_id: row.get(4)
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

pub fn tags_for_ids(ids: Vec<i32>, vendor_product_id: i32) -> Vec<models::Tag> {
    let conn = establish_connection();
    let mut tags: Vec<models::Tag> = Vec::new();

    match conn.query("SELECT id, name, creation_time, accessed_time, vendor_product_id FROM tag WHERE id = ANY($1) AND vendor_product_id = $2", &[&ids, &vendor_product_id]) {
        Ok(rows) => {
            for row in rows.iter() {
                let tag = models::Tag {
                    id: row.get(0),
                    name: row.get(1),
                    creation_time: row.get(2),
                    accessed_time: row.get(3),
                    vendor_product_id: row.get(4)
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

pub fn tag_new(name: String, vendor_product_id: i32) -> ds::TagItem {
    let conn = establish_connection();
    let mut tag = ds::TagItem {
        id: 0,
        name: name.to_owned(),
        creation_time: Utc::now(),
        accessed_time: Utc::now()
    };
    match conn.query("INSERT INTO tag(name, creation_time, accessed_time, vendor_product_id) VALUES($1,$2,$3,$4) RETURNING id",
     &[&tag.name, &tag.creation_time, &tag.accessed_time, &vendor_product_id]) {
         Ok(rows) => {
             let id:i32 = rows.iter().next().unwrap().get(0);
             tag.id = id;
             tag
         },
         Err(e) => {
            println!("Could not add new tag with name {}, error {}",name, e);
            tag
         }
     }
}

pub fn tag_update(id: i32, name: String, vendor_product_id: i32) {
    let conn = establish_connection();
    let tag = ds::TagItem {
        id: id,
        name: name.to_owned(),
        creation_time: Utc::now(),
        accessed_time: Utc::now()
    };
    conn.execute("UPDATE tag SET name = $1, accessed_time = $2 WHERE id = $3 RETURNING id",
     &[&tag.name, &tag.accessed_time, &tag.id])
     .expect(&format!("Could not update tag with name {} and id {}",name, id));
}

pub fn tag_index(id: i32, index_id: i32, vendor_product_id: i32) {
    let conn = establish_connection();
    conn.execute("UPDATE index SET tag_id = $1 WHERE id = $2",
     &[&id, &index_id])
     .expect(&format!("Could not add index with tag_id {} for id {}",id, index_id));

    conn.execute("UPDATE chunk SET tags = array_append(tags,$1) WHERE index_id = $2",
     &[&id, &index_id])
     .expect(&format!("Could not add chunk with tag_id {} for id {}",id, index_id));
}

pub fn tag_index_remove(id: i32, index_id: i32, vendor_product_id: i32) {
    let conn = establish_connection();
    conn.execute("UPDATE index SET tag_id = NULL WHERE id = $1",
     &[&index_id])
     .expect(&format!("Could not remove tag from index for id {}",index_id));

    conn.execute("UPDATE chunk SET tags = array_remove(tags,$1) WHERE index_id = $2",
     &[&id, &index_id])
     .expect(&format!("Could not remove tag_id {} from chunk for index_id {}",id, index_id));
}

pub fn indexes_for_ids(ids: Vec<i32>, vendor_product_id: i32) -> Vec<models::Index> {
    let conn = establish_connection();
    let mut indexes: Vec<models::Index> = Vec::new();

    match conn.query("SELECT id, name, chunks, creation_time, accessed_time,
                     stats_confirmed_download_count, stats_anonymous_download_count,vendor_product_id
                     FROM index WHERE id = ANY($1) AND vendor_product_id = $2", &[&ids, &vendor_product_id]) {
        Ok(rows) => {
            for row in rows.iter() {
                let index = models::Index {
                    id: row.get(0),
                    name: row.get(1),
                    chunks: row.get(2),
                    creation_time: row.get(3),
                    accessed_time: row.get(4),
                    stats_confirmed_download_count: row.get(5),
                    stats_anonymous_download_count: row.get(6),
                    vendor_product_id: row.get(7)
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

pub fn indexes_all(vendor_product_id: i32) -> Vec<models::Index> {
    let conn = establish_connection();
    let mut indexes: Vec<models::Index> = Vec::new();

    match conn.query("SELECT id, name, chunks, creation_time, accessed_time,
                     stats_confirmed_download_count, stats_anonymous_download_count,vendor_product_id
                     FROM index WHERE vendor_product_id = $1", &[&vendor_product_id]) {
        Ok(rows) => {
            for row in rows.iter() {
                let index = models::Index {
                    id: row.get(0),
                    name: row.get(1),
                    chunks: row.get(2),
                    creation_time: row.get(3),
                    accessed_time: row.get(4),
                    stats_confirmed_download_count: row.get(5),
                    stats_anonymous_download_count: row.get(6),
                    vendor_product_id: row.get(7)
                };
                indexes.push(index);
            }
        },
        Err(e) => {
            println!("Error getting all indexes, error {}", e);
        }
    };
    indexes
}

use ds::{IndexFile, IndexChunkItem};

pub fn insert_index(index: IndexFile, vendor_product_id: i32) -> Option<i32> {
    let conn = establish_connection();

    let chunks = index.chunks;
    let mut chunk_ids_inserted = Vec::new();
    let initial_download_count = 0;
    let mut index_id: i32 = 0;
    for index_chunk_item in chunks.iter() {
        let initial_tags: Vec<i32> = Vec::new();
        // TODO: Should have a field to indicate if the chunk is available
        match conn.query("INSERT INTO chunk(index_id, name, size,
                      creation_time, accessed_time, tags, stats_download_count,vendor_product_id)
                      VALUES( $1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
                         &[&index_id,&index_chunk_item.name, &index_chunk_item.size,
                           &Utc::now(), &Utc::now(), &initial_tags,
                           &initial_download_count, &vendor_product_id]) {
            Ok(rows) => {
                let id: i32 = rows.iter().next().unwrap().get(0);
                chunk_ids_inserted.push(id);
            },
            Err(e) => {
                println!("Could not insert chunks into chunk table");
            }
        }
    }
    match conn.query("INSERT INTO index(name, chunks, creation_time, accessed_time,
                  stats_confirmed_download_count, stats_anonymous_download_count, vendor_product_id)
                  VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id", 
                  &[&index.name,&chunk_ids_inserted,&Utc::now(), &Utc::now(),
                  &initial_download_count,&initial_download_count, &vendor_product_id]) {
        Ok(rows) => {
            index_id = rows.iter().next().unwrap().get(0);
        },
        Err(e) => {
            println!("Could not insert index into index table")
        }
    };
    conn.execute("UPDATE chunk SET index_id = $1 WHERE id = ANY($2)", &[&index_id, &chunk_ids_inserted])
        .expect("Could not update index_id back in chunk table");
    Some(index_id)
}

pub fn update_chunk_file_exists(index_id: i32, chunk_name: String, vendor_product_id: i32) {
    let conn = establish_connection();
    // TODO: If not a valid index_id, chunk_name, vendor_product_id, simply ignore
    conn.execute("UPDATE chunk SET file_exists = true 
                WHERE name = $1 AND vendor_product_id = $2 AND index_id = $3", 
                &[&chunk_name, &vendor_product_id, &index_id])
        .expect("Could not update index_id back in chunk table");
}

pub fn add_index_download_count(id: Option<i32>, name: Option<String>, c_count: i32, a_count: i32, vendor_product_id: i32) {
    let conn = establish_connection();
    match id {
        Some(id) => {
            if c_count > 0 || a_count > 0 {
                conn.execute("UPDATE index SET stats_anonymous_download_count = stats_anonymous_download_count + $1,
                                 stats_confirmed_download_count = stats_confirmed_download_count + $2 
                                WHERE id = $3 AND vendor_product_id = $4", 
                                &[&a_count, &c_count, &id, &vendor_product_id])
                    .expect("Could not update index stats");
            }
        },
        None => {
            if let Some(name) = name {
                if c_count > 0 || a_count > 0 {
                    conn.execute("UPDATE index SET stats_anonymous_download_count = stats_anonymous_download_count + $1,
                                     stats_confirmed_download_count = stats_confirmed_download_count + $2 
                                    WHERE name = $3 AND vendor_product_id = $4", 
                                    &[&a_count, &c_count, &name, &vendor_product_id])
                        .expect("Could not update index stats");
                }
            }
        }
    }
}

pub fn add_chunk_download_count(id: Option<i32>, name: Option<String>, c_count: i32, vendor_product_id: i32) {
    let conn = establish_connection();
    match id {
        Some(id) => {
            if c_count > 0 {
                conn.execute("UPDATE chunk SET stats_download_count = stats_download_count + $1 
                                WHERE id = $2 AND vendor_product_id = $3", 
                                &[&c_count, &id, &vendor_product_id])
                    .expect("Could not update chunk stats");
            }
        },
        None => {
            if let Some(name) = name {
                if c_count > 0 {
                    conn.execute("UPDATE chunk SET stats_download_count = stats_download_count + $1 
                                WHERE id = $2 AND vendor_product_id = $3", 
                                    &[&c_count, &name, &vendor_product_id])
                        .expect("Could not update chunk stats");
                }
            }
        }
    }
}