#[derive(FromForm)]
pub struct IndexUploadParams {
    pub name: String,
    pub version: String
}

#[derive(FromForm)]
pub struct ChunkUploadParams {
    pub name: String,
    pub index_id: i32
    // Can add sha field here
}

#[derive(FromForm)]
pub struct BlobUploadParams {
    pub blob_name: String,
    pub index_name: String
}

#[derive(FromForm)]
pub struct ChunksByIndexQueryParams {
    pub index_id: i32
}

#[derive(FromForm)]
pub struct ChunksByTagQueryParams {
    pub tag_id: i32
}

#[derive(FromForm)]
pub struct TagsByChunkQueryParams {
    pub chunk_id: i32
}

#[derive(FromForm)]
pub struct TagsByIndexQueryParams {
    pub index_id: i32
}

#[derive(FromForm)]
pub struct TagNewQueryParams {
    pub name: String
}

#[derive(FromForm)]
pub struct TagUpdateQueryParams {
    pub id: i32,
    pub name: String
}

#[derive(FromForm)]
pub struct TagIndexQueryParams {
    pub id: i32,
    pub index_id: i32
}

#[derive(FromForm)]
pub struct StatsIndexDownload {
    pub index_id: Option<i32>,
    pub index_name: Option<String>,
    pub confirmed_count: i32,
    pub anonymous_count: i32
}

#[derive(FromForm)]
pub struct StatsChunkDownload {
    pub chunk_id: Option<i32>,
    pub chunk_name: Option<String>,
    pub confirmed_count: i32
}