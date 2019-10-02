use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PriceDisplayType {
    TOTAL,
    M2,
}
