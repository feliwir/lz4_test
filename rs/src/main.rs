use std::{env, fs::File, io::{Read, Write}};

use lz4::block::{compress, CompressionMode};

fn encode(in_data: Vec<u8>) -> Vec<u8> {
    let original_size = in_data.len() as u32;
    let compressed_data = compress(&in_data, Some(CompressionMode::DEFAULT), false).unwrap();
    let compressed_size = (compressed_data.len() + 4) as u32;

    let compressed_size_bytes = compressed_size.to_le_bytes();
    let original_size_bytes = original_size.to_le_bytes();

    [
        [compressed_size_bytes, original_size_bytes].concat(),
        compressed_data,
    ]
    .concat()
}

fn main() {
    // Read file from the first argument
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file = File::open(filename).expect("File not found");

    // Read file into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    // Encode the buffer
    let encoded_buffer = encode(buffer);

    // Write the encoded buffer to the output parameter
    let mut output_file = File::create(&args[2]).expect("Failed to create output file");
    output_file
        .write_all(&encoded_buffer)
        .expect("Failed to write to output file");
}
