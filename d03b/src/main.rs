use std::{collections::HashMap, fs};

fn main() {
    let file = fs::read_to_string("input.txt").expect("File not found");

    let mut total = 0;
    let mut line_num = 0;
    let mut map = HashMap::<char, Vec<i32>>::new();
    for line in file.lines() {
        line_num += 1;
        println!("{}: {}", line_num, line);

        for c in line.chars() {
            if !map.contains_key(&c) {
                map.insert(c, vec![0, 0, 0]);
            }
            map.get_mut(&c).expect("Not found")[line_num % 3] += 1;
        }

        if line_num % 3 == 0 {
            for (key, val) in map.iter() {
                if val.iter().all(|x| *x != 0) {
                    println!("key: {} val: {:?}", *key, *val);
                    if key.is_lowercase() {
                        total += (*key as i32) - ('a' as i32) + 1;
                    } else {
                        total += (*key as i32) - ('A' as i32) + 27;
                    }
                }
            }
            map.clear();
        }
    }

    println!("{}", total);
}
