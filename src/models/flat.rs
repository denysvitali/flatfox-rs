use serde::{Serialize, Deserialize};

use crate::models::attribute::Attribute;
use crate::models::category::Category;
use crate::models::cover_image::CoverImage;
use crate::models::image::Image;
use crate::models::document::Document;
use crate::models::organization::Organization;
use crate::models::price_display_type::PriceDisplayType;

#[derive(Serialize, Deserialize, Debug)]
pub struct  Flat {
    pub attributes:Vec<Attribute>,
    pub category: Category,
    pub city: Option<String>,
    pub cover_image: Option<CoverImage>,
    pub cover_image_url: Option<String>,
    pub description: Option<String>,
    pub description_title: Option<String>,
    pub documents: Vec<Document>,
    pub floor: Option<i32>,
    pub id: i32,
    pub images: Vec<Image>,
    pub is_furnished: bool,
    pub is_selling_furniture: bool,
    pub is_temporary: bool,
    pub latitude: Option<f64>,
    pub livingspace: Option<i32>,
    pub longitude: Option<f64>,
    pub moving_date: Option<String>,
    pub moving_date_type: Option<String>,
    pub number_of_rooms: Option<String>,
    pub object_category: Option<String>,
    pub object_type: Option<String>,
    pub offer_type: Option<String>,
    pub organization: Option<Organization>,
    pub pk: i32,
    pub price_display: Option<i32>,
    pub price_display_type: PriceDisplayType,
    pub price_unit: Option<String>,
    pub public_address: Option<String>,
    pub public_subtitle: Option<String>,
    pub public_title: Option<String>,
    pub ref_house: Option<String>,
    pub ref_object: Option<String>,
    pub ref_property: Option<String>,
    pub rent_charges: Option<i32>,
    pub rent_display: Option<i32>,
    pub rent_gross: Option<i32>,
    pub rent_net: Option<i32>,
    pub rent_title: Option<String>,
    pub short_title: Option<String>,
    pub short_url: Option<String>,
    pub show_exact_address: bool,
    pub slug: Option<String>,
    pub status: Option<String>,
    pub thread_url: Option<String>,
    pub title: Option<String>,
    pub tour_url: Option<String>,
    pub url: Option<String>,
    pub video_url: Option<String>,
    pub website_url: Option<String>,
    pub year_built: Option<i32>,
    pub zipcode: Option<i32>
}