use concrete::{ConfigBuilder, generate_keys, set_server_key, FheUint16};
use concrete::{ServerKey, ClientKey};
use concrete::prelude::*;

use std::time::Instant;

fn save_server_key(data: ServerKey) {
    let mut file = std::io::BufWriter::new(std::fs::File::create("server_key.bin").unwrap());

    bincode::serialize_into(file, &data).unwrap();
}

fn save_client_key(data: ClientKey) {
    let mut file = std::io::BufWriter::new(std::fs::File::create("client_key.bin").unwrap());

    bincode::serialize_into(file, &data).unwrap();
}

fn read_server_key() -> ServerKey {
    let mut file = std::io::BufReader::new(std::fs::File::open("server_key.bin").unwrap());

    let de_data: ServerKey = bincode::deserialize_from(file).unwrap();

    de_data
}

fn read_client_key() -> ClientKey {
    let mut file = std::io::BufReader::new(std::fs::File::open("client_key.bin").unwrap());

    let de_data: ClientKey = bincode::deserialize_from(file).unwrap();

    de_data
}

fn save_ciphertext(data: Vec<FheUint16>) {
    let mut file = std::io::BufWriter::new(std::fs::File::create("ciphertext.bin").unwrap());

    bincode::serialize_into(file, &data).unwrap();
}

fn read_ciphertext() -> Vec<FheUint16> {
    let mut file = std::io::BufReader::new(std::fs::File::open("ciphertext.bin").unwrap());

    let de_data: Vec<FheUint16> = bincode::deserialize_from(file).unwrap();

    de_data
}

fn main() {   
    /* generate keys and save
    let config = ConfigBuilder::all_disabled()
        .enable_default_uint16()
        .build();

    let (client_key, server_key) = generate_keys(config);

    let test_a = 2.3 as f64;
    let test_b = 1.2 as f64;

    let clear_a = test_a.floor() as u16;
    let clear_b = test_b.floor() as u16;
    
    let tempt = server_key.clone();
    set_server_key(tempt);

    let a = FheUint16::encrypt(clear_a, &client_key);
    let b = FheUint16::encrypt(clear_b, &client_key);
    let mut ciphertext = vec![a, b];
   
    let start1 = Instant::now();
    save_server_key(server_key);
    let duration1 = start1.elapsed();
    println!("save server key: {:?}", duration1);

    let start2 = Instant::now();
    save_client_key(client_key);
    let duration2 = start2.elapsed();
    println!("save client key: {:?}", duration2);

    let start3 = Instant::now();
    save_ciphertext(ciphertext);
    let duration3 = start3.elapsed();
    println!("save ciphertexts: {:?}", duration3);
   */
   
   /* load and calaulate 
    let start1 = Instant::now();
    let mut new_server_key: ServerKey = read_server_key();
    let duration1 = start1.elapsed();
    println!("load server key: {:?}", duration1);

    let start2 = Instant::now();
    let mut new_client_key: ClientKey = read_client_key();
    let duration2 = start2.elapsed();
    println!("load client key: {:?}", duration2);

    let start3 = Instant::now();
    let mut new_ciphertext: Vec<FheUint16> = read_ciphertext();
    let duration3 = start3.elapsed();
    println!("load ciphertexts: {:?}", duration3);

    set_server_key(new_server_key);

    let new_a = new_ciphertext[0].clone();
    let new_b = new_ciphertext[1].clone();
    let result = &new_a + &new_b;

    let after_a: u16 = new_a.decrypt(&new_client_key);
    let after_b: u16 = new_b.decrypt(&new_client_key);
    let after_result: u16 = result.decrypt(&new_client_key);

    println!("a: {}", after_a);
    println!("b: {}", after_b);
    println!("a+b: {}", after_result);
   */
}



