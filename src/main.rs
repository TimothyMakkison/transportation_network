use learning_graph::parser::{parse_command, Command};
use std::fs;

fn main() {
    let commands = read_commands("Commands.txt");
    println!("{:?}", commands);

    let a = 0..10;
    for i in a.filter(|x| x % 2 == 0) {
        println!("{:?}", i);
    }
}

fn read_commands(path: &str) -> Result<Vec<Command>, &'static str> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut commands = vec![];
    for line in contents.lines() {
        println!("{:?}", line);

        let parsed = parse_command(line.as_bytes());
        commands.push(parsed.unwrap().1);
    }
    return Ok(commands);
}

fn process_command(command: Command) -> String {
    match command {
        Command::MaxDist => "max dist".to_string(),
        Command::FindNeighbour(place) => place.to_string(),
        _ => "Not impl".to_string(),
    }
}
