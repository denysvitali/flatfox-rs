use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Logo {
    pub url: String,
    pub url_org_logo_m: String,
}