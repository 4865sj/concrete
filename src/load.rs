use concrete::{ServerKey, ClientKey, FheUint16};
use concrete::prelude::*;

pub fn load_server_key() -> ServerKey {
    let mut file = std::io::BufReader::new(std::fs::File::open("server_key.bin").unwrap());

    let de_data: ServerKey = bincode::deserialize_from(file).unwrap();

    de_data
}

pub fn load_client_key() -> ClientKey {
    let mut file = std::io::BufReader::new(std::fs::File::open("client_key.bin").unwrap());

    let de_data: ClientKey = bincode::deserialize_from(file).unwrap();

    de_data
}

pub fn load_ciphertext() -> Vec<FheUint16> {
    let mut file = std::io::BufReader::new(std::fs::File::open("ciphertext.bin").unwrap());

    let de_data: Vec<FheUint16> = bincode::deserialize_from(file).unwrap();

    de_data
}

pub fn load_kernel() -> Vec<FheUint16> {
    let mut file = std::io::BufReader::new(std::fs::File::open("kernel.bin").unwrap());

    let de_data: Vec<FheUint16> = bincode::deserialize_from(file).unwrap();

    de_data
}
