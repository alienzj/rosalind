use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_gc_content(seq: String) -> f32 {
    let mut gc_count : i32 = 0;
    for nucleotide in seq.chars() {
        match nucleotide {
            'G' => gc_count += 1,
            'C' => gc_count += 1,
            _ => (),
        }
    }
    (gc_count as f32) / (seq.len() as f32) * 100.0
}

fn main() {
    let args : Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_handle = File::open(filename).expect("file not found");

    let mut seq = String::new();
    let mut id = String::from("");
    let mut gc_map = HashMap::new();
    let mut max_gc : f32 = 0.0;
    let mut max_gc_id = String::from("");

    for line in BufReader::new(file_handle).lines() {
        let l = line.unwrap();
        if l.starts_with(">") {
            if seq.len() != 0 {
                let gc = calculate_gc_content(seq);
                gc_map.insert(id.clone(), gc);
                if gc > max_gc {
                    max_gc = gc;
                    max_gc_id = id.clone();
                }
                seq = String::new();
            }
            id = l;
        } else {
            seq.push_str(&l);
        }
    }

    let gc = calculate_gc_content(seq);
    gc_map.insert(id.clone(), gc);
    if gc > max_gc {
        max_gc = gc;
        max_gc_id = id.clone();
    }

    for (key, value) in &gc_map {
        println!("{}: {}", key, value);
    }
    println!("\nMax GC content:\n{}: {}", max_gc_id, max_gc);
}
