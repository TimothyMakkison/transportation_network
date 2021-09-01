use learning_graph::startup::deserialization::{read_links, read_places};

fn main() {
    let places_path = "Places.csv";
    let links_path = "Links.csv";

    let places = read_places(places_path);

    for place in places {
        println!("{:?}", place);
    }

    let links = read_links(links_path);

    for link in links {
        println!("{:?}", link);
    }
}
