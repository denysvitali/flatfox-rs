use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Category {
    APARTMENT,
    SHARED,
    INDUSTRY,
    PARK,
    HOUSE,
}
