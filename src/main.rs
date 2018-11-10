use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Write;

extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    name: String,
    size: String
}

#[derive(Serialize, Deserialize)]
pub struct Index {
    id: u32,
    version: String,
    path: String,
    chunks: Vec<Chunk>
}

#[derive(Serialize, Deserialize)]
pub struct Indexes {
    indexes: Vec<Index>
}

impl Index {
    pub fn new(mut file: File, version: String, path: String) -> Index {
        let mut read_buf = [0; 70];
        let mut chunks = Vec::new();
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
            chunks.push(Chunk::new(chunk_file_name, format!("{}",uncompressed_chunk_size)));
        }
        Index {
            id: 1,
            version: version,
            path: path,
            chunks: chunks
        }
    }
}

impl Chunk {
    pub fn new(name: String, size: String) -> Chunk {
        Chunk {
            name: name,
            size: size
        }
    }
}

impl Indexes {
    pub fn new(indexes: Vec<Index>) -> Indexes {
        Indexes {
            indexes: indexes
        }
    }
}

fn main() {
    println!("Hello, world!");

}

fn add_index(file: File, version: String, path: String) {
    let index = index(file, version, path);
    let indexes = get_indexes();
    let mut indexes_vector = indexes.indexes;
    indexes_vector.push(index);
    let indexes_new = Indexes::new(indexes_vector);
    write_indexes(indexes_new);
}

fn index(file: File, version: String, path: String) -> Index {
    Index::new(file, version, path)
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

fn get_indexes() -> Indexes {
    let mut file = File::open(Path::new("test/indexes.json")).unwrap();
    let mut buffer = Vec::new();
    let read_len = file.read_to_end(&mut buffer).unwrap();
    let indexes: Indexes = serde_json::from_slice(&buffer).unwrap();
    indexes
}

fn write_indexes(indexes: Indexes) {
    let indexes_json = serde_json::to_vec(&indexes).unwrap();
    let mut file = File::create(Path::new("test/indexes.json")).unwrap();
    file.write_all(&indexes_json[..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_index() {
        let file = File::open(Path::new("test/index.caidx")).unwrap();
        add_index(file, ("0.1").to_string(), ("./").to_string());
    }

    #[test]
    fn test_index_new() {
        let file = File::open(Path::new("test/index.caidx")).unwrap();
        let index = index(file, ("0.1").to_string(), ("./").to_string());
        println!("index {:?}",index.chunks.len());
    }

    #[test]
    fn test_byte_array_to_u64() {
        let byte_array: [u8; 6] = [0,0,0,0,0,2];
        let value = byte_array_to_u64(byte_array);
        assert_eq!(value,2);

        let byte_array: [u8; 6] = [0,0,0,0,0,255];
        let value = byte_array_to_u64(byte_array);
        assert_eq!(value,255);

        let byte_array: [u8; 6] = [0,0,0,0,2,0];
        let value = byte_array_to_u64(byte_array);
        assert_eq!(value,512);

        let byte_array: [u8; 6] = [0,0,0,0,2,1];
        let value = byte_array_to_u64(byte_array);
        assert_eq!(value,513);

        let byte_array: [u8; 6] = [0,0,1,0,2,0];
        let value = byte_array_to_u64(byte_array);
        assert_eq!(value,16777728);
    }
}

