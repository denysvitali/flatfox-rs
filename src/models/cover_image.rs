use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoverImage {
    pub caption: Option<String>,
    pub height: i32,
    pub ordering: Option<i32>,
    pub pk: i32,
    pub search_url: Option<String>,
    pub url: Option<String>,
    pub url_listing_search: Option<String>,
    pub url_thumb_m: Option<String>,
    pub width: i32,
}
