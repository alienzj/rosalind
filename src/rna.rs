
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn transcribe(dna: String) -> String {
    let mut rna = String::new();
    rna.reserve(dna.len());

    for nucleotide in dna.chars() {
        match nucleotide {
            'T' => rna.push('U'),
            _ => rna.push(nucleotide)
        };
    }
    rna
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename).expect("file not found");
    let mut dna = String::new();
    f.read_to_string(&mut dna).expect("unable to read file");

    let rna = transcribe(dna);
    println!("{}", rna);

}
