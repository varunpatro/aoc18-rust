use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

#[macro_use] extern crate scan_fmt;


fn sol(v: &Vec<String>) -> (i32, i32) {
    let mut count = 0;
    let mut uniq = 0;
    let mut map = HashMap::new();
    let mut set = HashSet::new();

    for s in v {
        let (index, r, c, nr, nc) =
            scan_fmt!(s, "#{} @ {},{}: {}x{}", 
                i32, i32, i32, i32, i32);

        let (index, r, c, nr, nc) = 
            (index.unwrap(), r.unwrap(), c.unwrap(), nr.unwrap(), nc.unwrap());

        for i in c .. (c + nc) {
            for j in r .. (r + nr) {
                let e = map.entry((i, j)).or_insert(index);
                if *e != index {
                    set.insert(*e);
                    set.insert(index);
                    *e = 0;
                }
            }
        }
    }

    for v in map.values() {
        if *v == 0 {
            count += 1;
        }
    }


    for i in 1..v.len() as i32 + 1 {
        if !set.contains(&i) {
            uniq = i;
            break;
        }
    }

    (count, uniq)
}


fn main() {
    let stdin = io::stdin();
    let mut v = Vec::new();

    for line in stdin.lock().lines() {
        let s = line.unwrap();
        v.push(s);
    }

    let (n, u) = sol(&v);

    println!("num repeat: {}", n);
    println!("uniq index: {}", u);
}