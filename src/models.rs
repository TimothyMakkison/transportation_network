use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, PartialEq, Deserialize, Clone, Copy)]
pub enum TravelMode {
    Foot,
    Bike,
    Car,
    Bus,
    Ship,
    Rail,
}

impl Display for TravelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Link {
    pub start: i32,
    pub end: i32,
    pub mode: TravelMode,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Place {
    pub name: String,
    pub id: i32,
    latitude: f32,
    longitude: f32,
}

impl Place {
    pub fn new(name: String, id: i32, latitude: f32, longitude: f32) -> Self {
        Self {
            name,
            id,
            latitude,
            longitude,
        }
    }
}

impl Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PlaceCopy {
    pub id: i32,
    pub latitude: f32,
    pub longitude: f32,
}

impl PlaceCopy {
    pub fn from_place(source: Place) -> PlaceCopy {
        PlaceCopy {
            id: source.id,
            latitude: source.latitude,
            longitude: source.latitude,
        }
    }
}

impl Display for PlaceCopy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Command {
    MaxDist,
    MaxLink,
    FindDist(i32, i32),
    FindNeighbour(i32),
    Check(TravelMode, Vec<i32>),
    FindRoute(TravelMode, i32, i32),
    FindShortestRoute(TravelMode, i32, i32),
}
