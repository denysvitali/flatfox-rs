use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Pin {
    pub pk : i32,
    pub latitude: f64,
    pub longitude: f64,
    pub is_in_region: bool,
}