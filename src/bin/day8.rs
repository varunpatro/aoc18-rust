use std::io::{self, BufRead};

fn get_sum(v: &[usize], i: usize) -> (usize, usize, usize) {
    let mut sum = 0;
    let mut i = i;
    let mut values = Vec::new();
    let mut total_value = 0;
    let num_children = v[i];
    let num_entries = v[i + 1];

    i += 2;

    for _ in 0..num_children {
        let (s, new_i, value) = get_sum(v, i);
        values.push(value);
        sum += s;
        i = new_i;
    }


    let entries = &v[i..(i+num_entries)];
    let entries_sum = entries.iter().sum::<usize>();
    sum += entries_sum;

    if num_children == 0 {
        total_value = entries_sum;
    } else {
        for e in entries {
            let index = e - 1;
            if index < num_children {
                total_value += values[index];
            }
        }
    }

    i += num_entries;

    (sum, i, total_value)
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let v: Vec<usize> = line.unwrap().split(" ").map(|s| s.parse::<usize>().unwrap()).collect();
        let (sum, _, value) = get_sum(&v, 0);
        println!("sum: {}", sum);
        println!("total_value: {}", value);
    }
}