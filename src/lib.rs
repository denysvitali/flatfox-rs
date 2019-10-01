use crate::models::paginated::Paginated;
use crate::models::flat::Flat;
use std::borrow::{BorrowMut, Borrow};

pub mod models;

extern crate serde;

#[macro_use]
extern crate more_asserts;

use serde::{Serialize, Deserialize};
use crate::models::pin::Pin;
use reqwest::{Response, Url};
use crate::models::bounds::Bounds;
use std::str::FromStr;

const API_ENDPOINT : &str = "https://flatfox.ch/api";

fn make_request_str(url: &Url) -> String {
    reqwest::get(url.to_owned()).unwrap().text().unwrap()
}

pub fn get_flats() -> Paginated<Flat> {
    Paginated{
        count: 0,
        next: None,
        previous: None,
        results: vec![]
    }
}

pub fn get_pin(count: i32, bounds: Option<Bounds>, max_price: Option<i32>) -> Vec<Pin> {
    let mut url = Url::from_str(API_ENDPOINT).unwrap();
    url.path_segments_mut().unwrap().push("v1").push("pin");
    {
        let mut qp = url.query_pairs_mut();
        match bounds {
            Some(b) => {
                qp.append_pair("north", &b.north.to_string());
                qp.append_pair("south", &b.south.to_string());
                qp.append_pair("east", &b.east.to_string());
                qp.append_pair("west", &b.west.to_string());
            }
            _ => {}
        }

        match max_price {
            Some(m) => {
                qp.append_pair("max_price", &m.to_string());
            }
            _ => {}
        }
    }

    serde_json::from_str(&make_request_str(&url)).unwrap()
}

pub fn get_flat(pk: i32) -> Flat {
    let mut url = Url::from_str(API_ENDPOINT).unwrap();
    url.path_segments_mut().unwrap().push("v1").push("flat").push(&pk.to_string());
    serde_json::from_str(&make_request_str(&url)).unwrap()
}

pub fn get_flats_area(count: i32, bounds: Option<Bounds>, max_price: Option<i32>) -> Vec<Flat> {
    let pins = get_pin(count, bounds, max_price);
    let mut flats : Vec<Flat> = vec![];

    for p in pins {
        flats.push(get_flat(p.pk))
    }

    flats
}

#[cfg(test)]
mod tests {
    use crate::models::paginated::Paginated;
    use crate::models::flat::Flat;
    use crate::{get_pin, get_flat, get_flats_area};
    use crate::models::bounds::Bounds;

    #[test]
    fn parse_flat() {
        let flat_result: Paginated<Flat> =
            serde_json::from_str(include_str!("../resources/api/flat.json")).unwrap();
    }

    #[test]
    fn test_get_pin() {
        let pins = get_pin(10, Some(Bounds {
            north: 47.4554872,
            south: 47.2027764,
            west: 8.5439301,
            east: 8.7732697
        }), Some(1800));
        assert_le!(10, pins.len());
        println!("Pins: {:?}", pins);
    }

    #[test]
    fn test_get_flats_area() {
        let bounds = Bounds{
            north: 47.4554872,
            south: 47.2027764,
            west: 8.5439301,
            east: 8.7732697,
        };

        let flats = get_flats_area(10, Some(bounds), Some(1800));
        println!("Flats: {:?}", flats);
    }

    #[test]
    fn test_get_flat() {
        let f = get_flat(128617);
        assert_eq!(128617, f.id);
    }
}
