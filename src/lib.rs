use crate::models::flat::Flat;
use crate::models::paginated::Paginated;
use std::borrow::{Borrow, BorrowMut};

pub mod models;

extern crate serde;
extern crate strum;

#[macro_use]
extern crate more_asserts;

#[macro_use]
extern crate strum_macros;

use crate::models::bounds::Bounds;
use crate::models::pin::{Pin, PinSearch};
use crate::strum::AsStaticRef;
use reqwest::{Response, Url};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

const API_ENDPOINT: &str = "https://flatfox.ch/api";

fn make_request_str(url: &Url) -> String {
    reqwest::get(url.to_owned()).unwrap().text().unwrap()
}

pub fn get_flats() -> Paginated<Flat> {
    Paginated {
        count: 0,
        next: None,
        previous: None,
        results: vec![],
    }
}

pub fn get_pin(count: i32, bounds: Option<Bounds>, search: PinSearch) -> Vec<Pin> {
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

        qp.append_pair("limit", &count.to_string());

        match search.min_rooms() {
            Some(v) => {
                qp.append_pair("min_rooms", &v.to_string());
            }
            None => {}
        }

        match search.max_rooms() {
            Some(v) => {
                qp.append_pair("max_rooms", &v.to_string());
            }
            None => {}
        }

        match search.min_year_built() {
            Some(v) => {
                qp.append_pair("min_year_built", &v.to_string());
            }
            None => {}
        }

        match search.max_year_built() {
            Some(v) => {
                qp.append_pair("max_year_built", &v.to_string());
            }
            None => {}
        }

        match search.is_furnished() {
            Some(v) => {
                qp.append_pair("is_furnished", &v.to_string());
            }
            None => {}
        }

        match search.is_temporary() {
            Some(v) => {
                qp.append_pair("is_temporary", &v.to_string());
            }
            None => {}
        }

        match search.min_price() {
            Some(v) => {
                qp.append_pair("min_price", &v.to_string());
            }
            None => {}
        }

        match search.max_price() {
            Some(v) => {
                qp.append_pair("max_price", &v.to_string());
            }
            None => {}
        }

        match search.object_category() {
            Some(v) => {
                qp.append_pair("object_category", &v.as_static());
            }
            None => {}
        }
    }

    serde_json::from_str(&make_request_str(&url)).unwrap()
}

pub fn get_flat(pk: i32) -> Flat {
    let mut url = Url::from_str(API_ENDPOINT).unwrap();
    url.path_segments_mut()
        .unwrap()
        .push("v1")
        .push("flat")
        .push(&pk.to_string());
    serde_json::from_str(&make_request_str(&url)).unwrap()
}

pub fn get_flats_area(count: i32, bounds: Option<Bounds>, search: PinSearch) -> Vec<Flat> {
    let pins = get_pin(count, bounds, search);
    let mut flats: Vec<Flat> = vec![];

    for p in pins {
        flats.push(get_flat(p.pk))
    }

    flats
}

#[cfg(test)]
mod tests {
    use crate::models::bounds::Bounds;
    use crate::models::flat::{Flat, ObjectCategory};
    use crate::models::paginated::Paginated;
    use crate::models::pin::PinSearch;
    use crate::{get_flat, get_flats_area, get_pin};

    #[test]
    fn parse_flat() {
        let _flat_result: Paginated<Flat> =
            serde_json::from_str(include_str!("../resources/api/flat.json")).unwrap();
    }

    #[test]
    fn test_get_pin() {
        let mut search = &mut PinSearch::new();
        search = search
            .set_min_rooms(Some(1.0))
            .set_max_rooms(Some(3.5))
            .set_max_price(Some(1800))
            .set_is_temporary(Some(false));

        let search_res = search.to_owned().build();

        let pins = get_pin(
            10,
            Some(Bounds {
                north: 47.4554872,
                south: 47.2027764,
                west: 8.5439301,
                east: 8.7732697,
            }),
            search_res,
        );
        assert_le!(10, pins.len());
        println!("Pins: {:?}", pins);
    }

    #[test]
    fn test_get_flats_area() {
        let mut search = &mut PinSearch::new();
        search = search
            .set_min_rooms(Some(1.0))
            .set_max_rooms(Some(3.5))
            .set_max_price(Some(1800))
            .set_is_temporary(Some(false))
            .set_object_category(Some(ObjectCategory::APARTMENT));

        let search_res = search.to_owned().build();

        let bounds = Bounds {
            north: 47.4554872,
            south: 47.2027764,
            west: 8.5439301,
            east: 8.7732697,
        };

        let flats = get_flats_area(10, Some(bounds), search_res);
        println!("Flats:");

        for flat in flats {
            println!("\t{}", flat);
        }
    }

    #[test]
    fn test_get_flat() {
        let f = get_flat(128617);
        assert_eq!(128617, f.id);
    }
}
