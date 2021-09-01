use std::fs::{self};

use learning_graph::startup::parser::parse_command;

fn main() {
    let contents =
        fs::read_to_string("Commands.txt").expect("Something went wrong reading the file");

    let lines = contents.lines();

    for line in lines {
        println!("{:?}", line);

        let parsed = parse_command(line.as_bytes());
        println!("{:?}", parsed.expect("Error parsing").1);
    }
}
