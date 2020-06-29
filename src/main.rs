use std::fs::File;
use bit_vec::BitVec;
use std::io::{Write, Read, BufReader};
use std::slice;

#[repr(C, packed)]
#[derive(Debug, Clone)]
struct SavedBloomFilter {
    number_of_hash_functions: u8,
    bit_vector: BitVec
}

fn main() {
    let bloom_filter_to_save = SavedBloomFilter{ number_of_hash_functions: 2, bit_vector: BitVec::from_elem(128, false)};
    println!("hash_functions: {}, capacity: {}", bloom_filter_to_save.number_of_hash_functions, bloom_filter_to_save.bit_vector.capacity());
    let mut output_file = File::create("foo.bin").expect("Unable to open file");
    output_file.write(slice::from_ref(&bloom_filter_to_save.number_of_hash_functions));
    output_file.write(&bloom_filter_to_save.bit_vector.to_bytes());

    let input_file = File::open("foo.bin").expect("Unable to open input file");
    let mut reader = BufReader::new(input_file);
    let mut number_of_hash_functions = 0u8;

    reader.read(slice::from_mut(&mut number_of_hash_functions));
    let mut bytes: Vec<u8> = Vec::new();
    reader.read_to_end(&mut bytes);
    println!("read number of hash fuctions: {}", number_of_hash_functions);
    let saved_bloom_filter = SavedBloomFilter{ number_of_hash_functions: number_of_hash_functions, bit_vector: BitVec::from_bytes(&bytes)};
    println!("hash_functions: {}, capacity: {}", saved_bloom_filter.number_of_hash_functions, saved_bloom_filter.bit_vector.capacity());

}
