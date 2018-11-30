use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;
use self::models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn chunks_from_index_id(index_id_local: i32) -> Vec<Chunk> {
    use self::schema::chunk::dsl::*;
    let connection = establish_connection();
    let chunks_table: Vec<Chunk>  = chunk.filter(index_id.eq(&index_id_local)).load::<Chunk>(&connection).expect("Error loading chunks");
    chunks_table
}

pub fn tags_from_ids(tag_ids: Vec<i32>) -> Vec<Tag> {
    use self::schema::tag::dsl::*;
    let connection = establish_connection();
    let tags_table = tag.filter(id.eq(1)).load::<Tag>(&connection).expect("Error loading tags");
    tags_table
}

