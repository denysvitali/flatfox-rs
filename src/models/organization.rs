use crate::models::logo::Logo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Organization {
    pub name: String,
    pub logo: Option<Logo>,
}
