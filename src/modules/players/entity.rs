use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: u64,
    pub club_id: u64,
    pub name: String,
    pub position: String,
    pub age: i32,
}

#[derive(Deserialize)]
pub struct PlayerReq {
    pub club_id: Option<u64>,
    pub name: Option<String>,
}
