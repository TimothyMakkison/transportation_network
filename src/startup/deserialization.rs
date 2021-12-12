use std::fs;

use csv::ReaderBuilder;

use crate::{
    models::{Command, Link, Place, PlaceDto},
    startup::parser::parse_command,
};

pub fn read_places(path: &str) -> Vec<Place> {
    let mut rdr = ReaderBuilder::default()
        .has_headers(false)
        .from_path(path)
        .unwrap();

    let mut collection = vec![];
    for result in rdr.deserialize() {
        let mut record: PlaceDto = result.unwrap();

        // TODO. Have to subtract 1 such that all places all have a longitude of less than 0.
        // Otherwise the few places with a positive longitude will have highly inaccurate eastings.
        record.longitude -= 1.0;

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

pub fn read_commands(path: &str) -> Result<Vec<Command>, String> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut commands = vec![];
    for line in contents.lines() {
        let parsed = parse_command(line.as_bytes());
        if let Ok(command) = parsed {
            commands.push(command.1);
        }
    }
    Ok(commands)
}
