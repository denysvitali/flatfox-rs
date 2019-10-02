use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub enum Category {
    APARTMENT,
    SHARED,
    INDUSTRY,
    PARK,
    HOUSE,
}
