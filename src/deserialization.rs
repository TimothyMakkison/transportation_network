use std::fs;

use csv::ReaderBuilder;

use crate::{
    models::{Command, Link, Place, PlaceDto},
    parser::parse_command,
};

pub fn read_places(path: &str) -> Vec<Place> {
    let mut rdr = ReaderBuilder::default()
        .has_headers(false)
        .from_path(path)
        .unwrap();

    let mut collection = vec![];
    for result in rdr.deserialize() {
        let record: PlaceDto = result.unwrap();

        collection.push(record.into_place());
    }
    collection
}

pub fn read_links(path: &str) -> Vec<Link> {
    let mut rdr = ReaderBuilder::default()
        .has_headers(false)
        .from_path(path)
        .unwrap();

    let mut collection = vec![];
    for result in rdr.deserialize() {
        let record: Link = result.unwrap();
        collection.push(record);
    }

    collection
}

pub fn read_commands(path: &str) -> Result<Vec<Command>, &'static str> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut commands = vec![];
    for line in contents.lines() {
        let parsed = parse_command(line.as_bytes());
        commands.push(parsed.unwrap().1);
    }
    return Ok(commands);
}
