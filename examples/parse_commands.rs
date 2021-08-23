use learning_graph::parser::parser;

// fn main() {
//     let c1 = "MaxDist";
//     let c2 = "MaxLink";

//     let a = parser("MaxLink");
//     let b = parser("MaxDist");

//     println!("{:?}", a.unwrap().1);
//     println!("{:?}", b.unwrap().1);
// }
use std::fs::{self};

fn main() {
    let contents =
        fs::read_to_string("Commands.txt").expect("Something went wrong reading the file");

    let lines = contents.lines();

    for line in lines {
        println!("{:?}", line);

        let parsed = parser(line.as_bytes());
        println!("{:?}", parsed.unwrap().1);
    }
}
