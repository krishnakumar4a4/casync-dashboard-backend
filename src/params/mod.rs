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