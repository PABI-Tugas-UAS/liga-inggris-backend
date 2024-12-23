use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Match {
    pub id: u64,
    pub home_club_id: u64,
    pub away_club_id: u64,
    pub date: String,
    pub time: String,
    pub location: String,
    pub status: String,
    pub home_score: i32,
    pub away_score: i32,
}

#[derive(Deserialize)]
pub struct MatchReq {
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MatchStats {
    pub id: u64,
    pub match_id: u64,
    pub possession: String,
    pub shots_on_target: i32,
    pub passes: i32,
    pub corners: i32,
    pub fouls: i32,
}
