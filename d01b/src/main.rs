use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("File not found");

    let mut elfes = Vec::<u32>::new();
    elfes.push(0);
    for line in file.lines() {
        if line != "" {
            *elfes.last_mut().expect("not empty") += line.parse::<u32>().expect("ok");
        } else {
            elfes.push(0);
        }
    }

    elfes.sort();

    println!("{}", elfes.iter().rev().take(3).sum::<u32>());
}
