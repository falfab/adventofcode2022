use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Should be found");

    let mut cargo: Vec<Vec<char>> = vec![
        vec![], // 1
        vec![], // 2
        vec![], // 3
        vec![], // 4
        vec![], // 5
        vec![], // 6
        vec![], // 7
        vec![], // 8
        vec![], // 9
    ];

    for line in file.lines() {
        // Populate the cargo
        if line.starts_with("[") {
            for i in (1..line.len()).step_by(4) {
                let l = line.chars().nth(i).expect("Should be in range");
                if l != ' ' {
                    cargo[i / 4].push(l);
                }
            }
        }

        // Perform operations
        if line.starts_with("move") {
            let l = line
                .replace("move ", "")
                .replace("from ", "")
                .replace("to ", "");
            let v = l.split(" ").collect::<Vec<&str>>();
            let operations: Vec<u32> = v
                .iter()
                .map(|x| x.parse::<u32>().expect("Should be a number"))
                .collect();

            let quantity = operations[0] as usize;
            let from = (operations[1] - 1) as usize;
            let to = (operations[2] - 1) as usize;

            let load: Vec<char> = cargo[from][..quantity].iter().copied().rev().collect();
            cargo[to] = [load, cargo[to].clone()].concat();

            for _ in 0..quantity {
                cargo[from].remove(0);
            }
        }
    }
    for column in cargo {
        print!("{}", column.first().expect("Should not be empty"));
    }
    print!("\n");
}
