use csv::ReaderBuilder;
use learning_graph::models::{Link, Place};

fn main() {
    let places_path = "Places.csv";
    let links_path = "Links.csv";

    let mut rdr = ReaderBuilder::default()
        .has_headers(false)
        .from_path(places_path)
        .unwrap();

    for result in rdr.deserialize() {
        let record: Place = result.unwrap();
        println!("{:?}", record);
    }

    let mut rdlink = ReaderBuilder::default()
        .has_headers(false)
        .from_path(links_path)
        .unwrap();

    for result in rdlink.deserialize() {
        let record: Link = result.unwrap();
        println!("{:?}", record);
    }
}
