use petgraph::{graph::NodeIndex, visit::EdgeRef, Graph};
use std::fs;

fn create_game(
    graph: &mut Graph<&str, i32>,
    win_points: i32,
    draw_points: i32,
    loose_points: i32,
) -> (NodeIndex, NodeIndex, NodeIndex) {
    let rock = graph.add_node("A");
    let paper = graph.add_node("B");
    let scissors = graph.add_node("C");

    graph.add_edge(rock, scissors, win_points);
    graph.add_edge(paper, rock, win_points);
    graph.add_edge(scissors, paper, win_points);

    graph.add_edge(rock, rock, draw_points);
    graph.add_edge(paper, paper, draw_points);
    graph.add_edge(scissors, scissors, draw_points);

    graph.add_edge(scissors, rock, loose_points);
    graph.add_edge(rock, paper, loose_points);
    graph.add_edge(paper, scissors, loose_points);

    (rock, paper, scissors)
}

fn main() {
    // Define the rules of the game
    let mut game = Graph::<&str, i32>::new();
    let win_points = 6;
    let draw_points = 3;
    let loose_points = 0;
    let (rock, paper, scissors) = create_game(&mut game, win_points, draw_points, loose_points);
    let translate_game = |s: &str| match s {
        "A" => rock,
        "B" => paper,
        "C" => scissors,
        &_ => todo!(),
    };

    // Parse the file
    let file = fs::read_to_string("strategy.txt").expect("File not found");

    let mut score = 0;
    for line in file.lines() {
        let l: Vec<&str> = line.split_whitespace().collect();
        let opponent = translate_game(l[0]);
        let strategy = match l[1] {
            "X" => loose_points,
            "Y" => draw_points,
            "Z" => win_points,
            _ => todo!(),
        };

        let get_node = || -> Option<NodeIndex> {
            for node in game.neighbors(opponent) {
                let target = game
                    .edges(node)
                    .filter(|edge| *edge.weight() == strategy)
                    .next()
                    .expect("wtf")
                    .target();
                if target == opponent {
                    return Some(node);
                }
            }
            None
        };

        score += match game[get_node().expect("sad")] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => todo!(),
        };

        score += strategy;
    }

    println!("{score}");
}
