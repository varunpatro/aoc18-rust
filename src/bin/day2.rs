use std::io::{self, BufRead};
use std::collections::HashMap;

fn sol1(v: &Vec<String>) -> i32 {
    let mut two_counts = 0;
    let mut three_counts = 0;

    for s in v {
        let mut map = HashMap::new();
        for c in s.chars() {
            let e = map.entry(c).or_insert(0);
            *e += 1;
        }

        let mut two_count = 0;
        let mut three_count = 0;

        for val in map.values() {
            match val {
                2 => two_count |= 1,
                3 => three_count |= 1,
                _ => (),
            }
        }

        two_counts += two_count;
        three_counts += three_count;
    }

    return two_counts * three_counts;
}

fn one_off_char(a: &String, b: &String) -> Option<String> {
    let mut num_diff = 0;
    let mut index_diff = 0;

    if a.len() != b.len() {
        return None;
    }

    for (i, (x, y)) in a.chars().zip(b.chars()).enumerate() {
        if x != y {
            num_diff += 1;
            index_diff = i;
        }
    }

    if num_diff == 1 {
        let mut s = String::new();
        for (i, c) in a.chars().enumerate() {
            if i != index_diff {
                s.push(c);
            }
        }

        return Some(s);
    }

    return None;

}

fn sol2(v: &Vec<String>) -> String {
    for s1 in v {
        for s2 in v {
            match one_off_char(s1, s2) {
                Some(s) => return s,
                None => (),
            }
        }
    }

    return String::new();
}

fn main() {
    let stdin = io::stdin();
    let mut v = Vec::new();


    for line in stdin.lock().lines() {
        let s = line.unwrap();
        v.push(s);
    }

    println!("checksum: {}", sol1(&v));
    println!("common: {}", sol2(&v));
}