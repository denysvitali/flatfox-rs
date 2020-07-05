use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PriceDisplayType {
    TOTAL,
    M2,
}
