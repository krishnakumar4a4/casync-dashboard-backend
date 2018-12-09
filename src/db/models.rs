use chrono::prelude::{DateTime, Utc};

pub struct Vendor {
    pub id: i32,
    pub name: String,
    pub creation_time: DateTime<Utc>,
    pub accessed_time: DateTime<Utc>
}

pub struct Product {
    pub id: i32,
    pub name: String,
    pub creation_time: DateTime<Utc>,
    pub accessed_time: DateTime<Utc>
}

pub struct VendorProduct {
    pub id: i32,
    pub vendor_id: i32,
    pub product_id: i32,
    pub creation_time: DateTime<Utc>,
    pub accessed_time: DateTime<Utc>
}

pub struct Tag {
    pub id: i32,
    pub name: String,
    pub creation_time: DateTime<Utc>,
    pub accessed_time: DateTime<Utc>,
    pub vendor_product_id: i32
}

pub struct Chunk {
    pub id: i32,
    pub index_id: i32,
    pub name: String,
    pub size: i32,
    pub creation_time: DateTime<Utc>,
    pub accessed_time: DateTime<Utc>,
    pub tags: Option<Vec<i32>>,
    pub stats_download_count: i32,
    pub vendor_product_id: i32
}

pub struct Index {
    pub id: i32,
    pub name: String,
    pub chunks: Option<Vec<i32>>,
    pub creation_time: DateTime<Utc>,
    pub accessed_time: DateTime<Utc>,
    pub stats_confirmed_download_count: i32,
    pub stats_anonymous_download_count: i32,
    pub vendor_product_id: i32
}
