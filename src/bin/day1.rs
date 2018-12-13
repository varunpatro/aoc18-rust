use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let mut v = Vec::new();
    for line in stdin.lock().lines() {
    	let freq: i32 = line.unwrap().parse().unwrap();
    	v.push(freq);
    }

    let total_freq: i32 = v.iter().sum();

    let mut cum_freq = 0;
    let mut cum_freqs_so_far = HashSet::new();
    let mut i = 0;
    while !cum_freqs_so_far.contains(&cum_freq) {
    	cum_freqs_so_far.insert(cum_freq);
    	cum_freq += v[i];
    	i += 1;
    	i = i % v.len();
    }

    println!("cumulative frequency: {}", total_freq);
    println!("first repeated freq: {}", cum_freq);
}