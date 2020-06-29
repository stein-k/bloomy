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
    let saved_bloom_filter = SavedBloomFilter{ number_of_hash_functions: 2, bit_vector: BitVec::from_elem(128, false)};
    println!("hash_functions: {}, set_bits: {}", saved_bloom_filter.number_of_hash_functions, saved_bloom_filter.bit_vector.capacity());
    let mut output_file = File::create("foo.bin").expect("Unable to open file");
    output_file.write(slice::from_ref(&saved_bloom_filter.number_of_hash_functions));
    output_file.write(&saved_bloom_filter.bit_vector.to_bytes());
    println!("Hello, world!");

    let input_file = File::open("foo.bin").expect("Unable to open input file");
    let mut reader = BufReader::new(input_file);
    let mut number_of_hash_functions = 0u8;

    reader.read(slice::from_mut(&mut number_of_hash_functions));
    println!("read number of hash fuctions: {}", number_of_hash_functions)

}
