use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
enum TransportType {
    Foot,
    Bike,
    Car,
    Bus,
    Rail,
    Ship,
}
#[derive(Debug, Deserialize)]
struct Link {
    start: i32,
    end: i32,
    mode: TransportType,
}

#[derive(Debug, Deserialize)]
struct Place {
    name: String,
    id: i32,
    latitude: f32,
    longitude: f32,
}

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
