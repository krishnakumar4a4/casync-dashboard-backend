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

pub fn chunks_from_index_id(index_id_local: i32) -> Vec<Chunk>{
    use self::schema::chunks::dsl::*;
    let connection = establish_connection();

    let chunks_table  = chunks.filter(index_id.eq(&index_id_local)).load::<Chunk>(&connection).expect("Error loading chunks");
    //let chunks_table  = chunks.load::<Chunk>(&connection).expect("Error loading chunks");
    //let chunks = chunks_table::all_columns();
    for chunk in chunks_table.iter() {
        println!("Chunk id: {}", chunk.id);
    }
    chunks_table
}

