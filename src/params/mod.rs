#[derive(FromForm)]
pub struct IndexUploadParams {
    pub name: String,
    pub version: String
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