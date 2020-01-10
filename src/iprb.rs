use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn probability(k : u32, m : u32, n : u32) -> f32 {
    let poputation_total = k + m + n;
    let posibility_total = 4 * poputation_total * (poputation_total - 1);

    let dd = k * (k - 1);
    let dh = 2 * k * m;
    let dr = 2 * k * n;

    let hh = m * (m - 1);
    let hr = 2 * m * n;

    let d_total = 4 * dd + 4 * dh + 4 * dr + 3 * hh + 2 * hr;
    d_total as f32 / posibility_total as f32
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let f = File::open(&args[1]).expect("file not found");

    println!("{}", probability(2, 2, 2));

    for line in BufReader::new(f).lines() {
        let l = line.unwrap();
        let nums : Vec<&str> = l.trim().split(' ').collect();
        let k : u32 = nums[0].parse().unwrap();
        let m : u32 = nums[1].parse().unwrap();
        let n : u32 = nums[2].parse().unwrap();
        println!("{}", probability(k, m, n));
    }
}
