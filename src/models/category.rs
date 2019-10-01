use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum Category {
    APARTMENT, SHARED, INDUSTRY, PARK, HOUSE
}