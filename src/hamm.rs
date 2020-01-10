use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

struct Pairer<T: Iterator> {
    it: T,
    prev: Option<T::Item>,
}

impl<T: Iterator> Pairer<T> {
    pub fn new(it: T) -> Self {
        Pairer { it, prev: None }
    }
}

impl<T: Iterator> Iterator for Pairer<T> {
    type Item = (T::Item, T::Item);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let first = self.it.next();
        let second = self.it.next();

        match (first, second) {
            (None, _) => None,
            (Some(_), None) => None,
            (Some(a), Some(b)) => Some((a, b)),
        }
    }
}

fn hamming_distance(seq_a: &str, seq_b: &str) -> u32 {
    let mut dist: u32 = 0;
    for (a, b) in seq_a.chars().zip(seq_b.chars()) {
        if a != b {
            dist += 1
        }
    }
    dist
}

fn hamming_distance2(seq_a: &str, seq_b: &str) -> usize {
    seq_a
        .chars()
        .zip(seq_b.chars())
        .filter(|c| c.0 != c.1)
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let pairs = File::open(filename)
        .map(io::BufReader::new)
        .expect("file not found")
        .lines()
        .filter_map(Result::ok);

    for (seq_a, seq_b) in Pairer::new(pairs) {
        println!("{}", hamming_distance2(&seq_a, &seq_b));
    }

    // method2
    let mut f = File::open(filename).expect("file not found");
    let mut dnas = String::new();
    f.read_to_string(&mut dnas).expect("unable to read file");
    let seqs = dnas.split("\n").collect::<Vec<&str>>();
    println!("{}", hamming_distance(&seqs[0], &seqs[1]));
}
