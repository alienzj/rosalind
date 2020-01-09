use std::env;
use std::fs::File;
use std::io::prelude::*;

fn count_nucleotides(dna: String) -> [u64; 5] {
    let mut counts: [u64; 5] = [0; 5];
    for nucleotide in dna.chars() {
        match nucleotide {
            'A' => counts[0] += 1,
            'C' => counts[1] += 1,
            'G' => counts[2] += 1,
            'T' => counts[3] += 1,
            _ => counts[4] += 1,
        }
    }
    counts
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut dna = String::new();
    f.read_to_string(&mut dna).expect("unable to read file");

    let counts: [u64; 5] = count_nucleotides(dna);

    println!("A: {}", counts[0]);
    println!("C: {}", counts[1]);
    println!("G: {}", counts[2]);
    println!("T: {}", counts[3]);
    println!("Other: {}", counts[4]);
}
