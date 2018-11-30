//use chrono::DateTime;
use chrono::prelude::{DateTime, Utc};

#[derive(Queryable)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub creation_time: DateTime<Utc>,
    pub accessed_time: DateTime<Utc>
}

#[derive(Queryable)]
pub struct Chunk {
    pub id: i32,
    pub index_id: i32,
    pub name: String,
    pub size: i32,
    pub creation_time: String,
    pub accessed_time: String,
    pub tags: Option<Vec<i32>>,
    pub stats_download_count: i32
}
