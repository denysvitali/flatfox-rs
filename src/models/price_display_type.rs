use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PriceDisplayType {
    TOTAL, M2
}