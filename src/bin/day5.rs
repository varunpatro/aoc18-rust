use std::io;
// use std::collections::{HashMap, HashSet};

fn collapse_string(s: &str) -> String {
    let mut new = String::new();
    let mut prev_char = ' ';
    for c in s.chars() {
        let diff = c as i32 - prev_char as i32;
        if diff == 32 || diff == -32 {
            prev_char = ' ';
        } else {
            if prev_char != ' ' {
                new.push(prev_char);
            }
            prev_char = c;
        }
    }

    if prev_char != ' ' {
        new.push(prev_char);
    }

    new
}

fn max_len(s: &String) -> usize {
    let mut prev = String::new();
    let mut next = s.clone();
    let len = loop {
        if prev == next {
            break next.len()
        }

        prev = next.clone();
        next = collapse_string(&next);
    };

    len - 1
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let len = max_len(&buffer);
    println!("len: {}", len);

    let mut min = len;

    for c in b'a'..b'z'+1 {
        let new_string = buffer.replace(c as char, "").replace((c-32) as char, "");
        min = min.min(max_len(&new_string));
    }

    println!("min: {}", min);

    Ok(())

}