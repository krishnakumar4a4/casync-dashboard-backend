use std::fs::File;
use std::path::Path;
use std::io::Read;
use chrono::{DateTime, Utc};
use db;

#[derive(Serialize, Deserialize)]
pub struct TagItem {
    pub id: i32,
    pub name: String,
    #[serde(with = "my_date_format")]
    pub creation_time: chrono::DateTime<Utc>,
    #[serde(with = "my_date_format")]
    pub accessed_time: chrono::DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
pub struct ChunkItem {
    id: i32,
    index_id: i32,
    name: String,
    size: i64,
    #[serde(with = "my_date_format")]
    creation_time: chrono::DateTime<Utc>,
    #[serde(with = "my_date_format")]
    accessed_time: chrono::DateTime<Utc>,
    tags: Vec<TagItem>,
    stats_download_count: i32
}

#[derive(Serialize, Deserialize)]
pub struct ChunkItemRow {
    id: i32,
    index_id: i32,
    name: String,
    size: i64,
    #[serde(with = "my_date_format")]
    creation_time: chrono::DateTime<Utc>,
    #[serde(with = "my_date_format")]
    accessed_time: chrono::DateTime<Utc>,
    tags: Vec<i32>,
    stats_download_count: i32
}

#[derive(Serialize, Deserialize)]
pub struct IndexItem {
    pub id: i32,
    pub name: String,
    pub chunks: Vec<ChunkItem>,
    #[serde(with = "my_date_format")]
    pub creation_time: chrono::DateTime<Utc>,
    #[serde(with = "my_date_format")]
    pub accessed_time: chrono::DateTime<Utc>,
    pub stats_confirmed_download_count: i32,
    pub stats_anonymous_download_count: i32
}

#[derive(Serialize, Deserialize)]
pub struct IndexItemRow {
    pub id: i32,
    pub name: String,
    #[serde(with = "my_date_format")]
    pub creation_time: chrono::DateTime<Utc>,
    #[serde(with = "my_date_format")]
    pub accessed_time: chrono::DateTime<Utc>,
    pub stats_confirmed_download_count: i32,
    pub stats_anonymous_download_count: i32
}

impl ChunkItemRow {
    pub fn new(id: i32, index_id: i32, name: String, size: i64, creation_time: chrono::DateTime<Utc>, accessed_time: chrono::DateTime<Utc>, tags: Vec<i32>, download_count: i32) -> ChunkItemRow {
        ChunkItemRow {
            id: id,
            index_id: index_id,
            name: name,
            size: size,
            creation_time: creation_time,
            accessed_time: accessed_time,
            tags: tags,
            stats_download_count: download_count
        }
    }
}

impl ChunkItem {
    pub fn new(id: i32, index_id: i32, name: String, size: i64, creation_time: chrono::DateTime<Utc>, accessed_time: chrono::DateTime<Utc>, tags: Vec<&db::models::Tag>, download_count: i32) -> ChunkItem {
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
    pub fn new(id: i32, name: String,
               chunks: Vec<ChunkItem>,
               creation_time: chrono::DateTime<Utc>,
               accessed_time: chrono::DateTime<Utc>,
               stats_confirmed_download_count: i32,
               stats_anonymous_download_count: i32) -> IndexItem {
        IndexItem {
            id: id,
            name: name,
            chunks: chunks,
            creation_time: creation_time,
            accessed_time: accessed_time,
            stats_confirmed_download_count: stats_confirmed_download_count,
            stats_anonymous_download_count: stats_anonymous_download_count
        }
    }
}

impl IndexItemRow {
    pub fn new(id: i32, name: String,
               creation_time: chrono::DateTime<Utc>,
               accessed_time: chrono::DateTime<Utc>,
               stats_confirmed_download_count: i32,
               stats_anonymous_download_count: i32) -> IndexItemRow {
        IndexItemRow {
            id: id,
            name: name,
            creation_time: creation_time,
            accessed_time: accessed_time,
            stats_confirmed_download_count: stats_confirmed_download_count,
            stats_anonymous_download_count: stats_anonymous_download_count
        }
    }
}

#[derive(Clone)]
pub struct IndexChunkItem {
    pub name: String,
    pub size: i64
}

pub struct IndexFile {
    pub name: String,
    pub version: String,
    pub path: String,
    pub chunks: Vec<IndexChunkItem>
}

impl IndexChunkItem {
    pub fn new(name: String, size: u64) -> IndexChunkItem {
        IndexChunkItem {
            name: name,
            size: (size as i64)
        }
    }
}

impl IndexFile {
    pub fn new(index_file_path: String, index_file: String, version: String) -> IndexFile {
        let mut full_index_file_path = index_file_path.clone();
        full_index_file_path.push_str("/");
        full_index_file_path.push_str(&(index_file.to_owned()));

        let mut read_buf = [0; 70];
        let mut chunks = Vec::new();
        let mut file = File::open(Path::new(&full_index_file_path)).unwrap();
        loop {
            match file.read_exact(&mut read_buf) {
                Ok(()) => (),
                Err(_err) => {
                    break;
                }
            };
            let chunk_file_name = String::from_utf8(read_buf[..64].to_vec()).unwrap();
            let mut uncompressed_chunk_size_bytes: [u8; 6] = [0; 6];
            uncompressed_chunk_size_bytes.copy_from_slice(&read_buf[64..70]);
            let uncompressed_chunk_size = byte_array_to_u64(uncompressed_chunk_size_bytes);
            let chunk = IndexChunkItem::new(chunk_file_name,uncompressed_chunk_size);
            chunks.push(chunk);
        }
        IndexFile {
            name: index_file,
            version: version,
            path: index_file_path,
            chunks: chunks
        }
    }
}

fn byte_array_to_u64(byte_array: [u8;6]) -> u64 {
    let mut i = 6;
    let mut value:u64 = 0;
    for b in byte_array.iter() {
        i=i-1;
        let radix_val: u64 = 256u64.pow(i);
        value = value + ((radix_val * (b.clone() as u64)) as u64);
    }
    value
}

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
