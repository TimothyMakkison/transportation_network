use coord_transforms::{geo::ll2utm, prelude::Vector2, structs::geo_ellipsoid};
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
pub struct PlaceDto {
    pub name: String,
    pub id: i32,
    pub latitude: f64,
    pub longitude: f64,
}

impl PlaceDto {
    pub fn into_place(self) -> Place {
        let utm_grid = self.convert_to_grid();

        Place::new(
            self.name,
            self.id,
            utm_grid.get_northing(),
            utm_grid.get_easting(),
        )
    }

    fn convert_to_grid(&self) -> coord_transforms::structs::utm_grid::utm_grid {
        let ellipsoid = geo_ellipsoid::geo_ellipsoid::new(
            geo_ellipsoid::WGS84_SEMI_MAJOR_AXIS_METERS,
            geo_ellipsoid::WGS84_FLATTENING,
        );
        let lat: f64 = self.latitude;
        let long: f64 = self.longitude;
        let ll_vec: Vector2<f64> = Vector2::new(lat.to_radians(), long.to_radians());
        let utm_grid = ll2utm(&ll_vec, &ellipsoid);
        utm_grid
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Place {
    pub name: String,
    pub id: i32,
    pub northings: f64,
    pub eastings: f64,
}

impl Place {
    pub fn new(name: String, id: i32, northings: f64, eastings: f64) -> Self {
        Self {
            name,
            id,
            northings,
            eastings,
        }
    }
}

impl Display for Place {
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
