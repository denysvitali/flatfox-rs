use crate::models::flat::ObjectCategory;
use getset::{Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pin {
    pub pk: i32,
    pub latitude: f64,
    pub longitude: f64,
    pub is_in_region: bool,
}

#[derive(Clone, Getters, Setters)]
#[get = "pub"]
#[set = "pub"]
pub struct PinSearch {
    min_rooms: Option<f32>,
    max_rooms: Option<f32>,

    min_space: Option<i32>,
    max_space: Option<i32>,

    min_floor: Option<i32>,
    max_floor: Option<i32>,

    min_price: Option<i32>,
    max_price: Option<i32>,

    min_year_built: Option<i32>,
    max_year_built: Option<i32>,

    is_furnished: Option<bool>,
    is_temporary: Option<bool>,

    object_category: Option<ObjectCategory>,
}

impl PinSearch {
    pub(crate) fn new() -> PinSearch {
        PinSearch {
            min_rooms: None,
            max_rooms: None,
            min_space: None,
            max_space: None,
            min_floor: None,
            max_floor: None,
            min_price: None,
            max_price: None,
            min_year_built: None,
            max_year_built: None,
            is_furnished: None,
            is_temporary: None,
            object_category: None,
        }
    }

    pub(crate) fn build(self) -> PinSearch {
        self.clone()
    }
}
