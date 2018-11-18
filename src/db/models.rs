#[derive(Queryable)]
pub struct Chunk {
    pub id: i32,
    pub index: String,
    pub name: String,
    pub size: i32
}
