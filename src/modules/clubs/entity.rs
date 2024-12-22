use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Club {
    pub id: u64,
    pub name: String,
    pub coach: String,
    pub founded_year: i32,
    pub achievements: Vec<String>,
    pub logo: String,
}

#[derive(Deserialize)]
pub struct ClubReq {
    pub name: Option<String>,
}
