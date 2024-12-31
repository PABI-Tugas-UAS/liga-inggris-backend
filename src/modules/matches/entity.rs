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
    pub timeline: Vec<Timeline>,
    pub stats: Stats,
}

#[derive(Serialize, Deserialize)]
pub struct Timeline {
    pub minute: u64,
    pub description: String,
    pub team: String,
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub passings: TeamStats,
    pub tackles: TeamStats,
    pub shots_on_target: TeamStats,
    pub fouls: TeamStats,
    pub corners: TeamStats,
}

#[derive(Serialize, Deserialize)]
pub struct TeamStats {
    pub home: u64,
    pub away: u64,
}
