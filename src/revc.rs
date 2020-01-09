use std::env;
use std::fs::File;
use std::io::prelude::*;

fn reverse_complement(dna: String) -> String {
    let mut dna_rc = String::new();
    dna_rc.reserve(dna.len());

    for nucleotide in dna.trim().chars().rev() {
        match nucleotide {
            'A' => dna_rc.push('T'),
            'C' => dna_rc.push('G'),
            'G' => dna_rc.push('C'),
            'T' => dna_rc.push('A'),
            _ => dna_rc.push('N'),
        }
    }
    dna_rc
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut dna = String::new();
    f.read_to_string(&mut dna).expect("unable to read file");

    let dna_rc = reverse_complement(dna);
    println!("{}", dna_rc);
}
