use std::{collections::HashSet, fs};

fn main() {
    let file = fs::read_to_string("input.txt").expect("File should be found");

    let mut total = 0;
    for line in file.lines() {
        let sections: Vec<&str> = line.split(",").collect();

        let mut values = vec![0, 0, 0, 0];
        let mut idx = 0;
        for section in sections {
            let nums: Vec<&str> = section.split("-").collect();

            for num in nums {
                values[idx] = num.parse::<i32>().expect("Num should be a number");
                idx += 1;
            }
        }

        // create the sets
        let set1: HashSet<i32> = (values[0]..values[1] + 1).collect();
        let set2: HashSet<i32> = (values[2]..values[3] + 1).collect();
        let intersect: HashSet<i32> = set1.intersection(&set2).cloned().collect();

        if !intersect.is_empty() {
            total += 1;
        }
    }
    println!("{}", total);
}
