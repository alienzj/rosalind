use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn fib(n: u64, k: u64) -> u64 {
    if n <= 1 { return 1 }
    fib(n - 1, k) + fib(n - 2, k) * k
}

fn fib2(n: u64, k: u64) -> u64 {
    let mut total : u64 = 0;
    if n == 0 {
        total = 0
    } else if n == 1 {
        total = 1
    } else {
        let mut i = n;
        let mut prev = 0;
        let mut next = 1;
        while i >= 2 {
            total = prev * k + next;
            prev = next;
            next = total;
            i -= 1;
        }
    }
    total
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_handle = File::open(filename).expect("file not found");

    println!("{}", fib2(5, 3));

    for line in BufReader::new(file_handle).lines() {
        let l = line.unwrap();
        let nums : Vec<&str> = l.split(' ').collect();
        println!("{} {}", nums[0], nums[1]);
        let n = nums[0].parse::<u64>();
        let k = nums[1].parse::<u64>();
        match (n, k) {
            (Ok(n_), Ok(k_)) => {
                println!("{}", fib(n_, k_));
            },
            (_, _) => {
                println!("Invalid input!");
                break;
            }
        }
    }
}
