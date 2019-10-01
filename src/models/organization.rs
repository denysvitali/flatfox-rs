use serde::{Serialize, Deserialize};
use crate::models::logo::Logo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Organization {
    pub name: String,
    pub logo: Option<Logo>,
}