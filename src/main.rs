use std::fs::File;
use bit_vec::BitVec;
use std::io::{Write, Read, BufReader};
use std::slice;

struct BloomFilter {
    number_of_hash_functions: u8,
    bit_vector: BitVec
}

impl BloomFilter {
    fn to_file(&self, file_path: String, bloom_filter_to_save: &BloomFilter) -> () {
        let mut output_file = File::create(file_path).expect("Unable to open output file");
        output_file.write(slice::from_ref(&bloom_filter_to_save.number_of_hash_functions)).expect("Unable to write number of hash functions");
        output_file.write(&bloom_filter_to_save.bit_vector.to_bytes()).expect("Unable to write bit vector");
    }

    fn from_file(file_path: String) -> BloomFilter {
        let input_file = File::open(file_path).expect("Unable to open input file");
        let mut reader = BufReader::new(input_file);

        let mut number_of_hash_functions: u8 = 0;
        let mut stored_bits: Vec<u8> = Vec::new();

        reader.read(slice::from_mut(&mut number_of_hash_functions)).expect("Unable to read number of hash functions");
        reader.read_to_end(&mut stored_bits).expect("Unable to read bit vector");

        BloomFilter { number_of_hash_functions: number_of_hash_functions, bit_vector: BitVec::from_bytes(&stored_bits)}
    }

}

fn main() {
    let bloom_filter_to_save = BloomFilter { number_of_hash_functions: 2, bit_vector: BitVec::from_elem(128, false)};
    println!("hash_functions: {}, capacity: {}", bloom_filter_to_save.number_of_hash_functions, bloom_filter_to_save.bit_vector.capacity());
    bloom_filter_to_save.to_file(String::from("foo.bin"), &bloom_filter_to_save);


    let saved_bloom_filter = BloomFilter::from_file(String::from("foo.bin"));
    println!("hash_functions: {}, capacity: {}", saved_bloom_filter.number_of_hash_functions, saved_bloom_filter.bit_vector.capacity());

}
