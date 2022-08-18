use concrete::{ServerKey, ClientKey, FheUint16};
use concrete::prelude::*;

pub fn save_server_key(data: ServerKey) {
    let mut file = std::io::BufWriter::new(std::fs::File::create("server_key.bin").unwrap());

    bincode::serialize_into(file, &data).unwrap();
}

pub fn save_client_key(data: ClientKey) {
    let mut file = std::io::BufWriter::new(std::fs::File::create("client_key.bin").unwrap());

    bincode::serialize_into(file, &data).unwrap();
}

pub fn save_ciphertext(data: Vec<FheUint16>) {
    let mut file = std::io::BufWriter::new(std::fs::File::create("ciphertext.bin").unwrap());

    bincode::serialize_into(file, &data).unwrap();
}

pub fn save_kernel(data: Vec<FheUint16>) {
    let mut file = std::io::BufWriter::new(std::fs::File::create("kernel.bin").unwrap());

    bincode::serialize_into(file, &data).unwrap();
