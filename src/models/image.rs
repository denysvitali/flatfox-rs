use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub caption: Option<String>,
    pub height: i32,
    pub ordering: i32,
    pub pk: i32,
    pub search_url: Option<String>,
    pub url: Option<String>,
    pub url_listing_search: Option<String>,
    pub url_thumb_m: Option<String>,
    pub width: i32,
}