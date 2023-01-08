use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Should be found");

    let line = file.lines().next().expect("At least one line");

    let mut buf: Vec<char> = vec![];
    let mut count = 0;
    for c in line.chars() {
        if buf.contains(&c) {
            let idx = buf.iter().position(|&x| x == c).unwrap() + 1;
            for _ in 0..idx {
                buf.remove(0);
            }
        }
        buf.push(c);
        count += 1;
        if buf.len() >= 14 {
            break;
        }
    }
    println!("Count: {}", count);
}
