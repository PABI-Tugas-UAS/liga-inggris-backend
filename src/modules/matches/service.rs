use super::{entity::MatchReq, repository};
use crate::modules::clubs::service as club_service;
use serde_json::Value;

pub fn get_matches(req: MatchReq) -> Vec<Value> {
    let matches = repository::get_matches(Some(req));

    matches
        .into_iter()
        .map(|club| {
            serde_json::json!({
                "id": club.id,
                "home_club_id": club.home_club_id,
                "away_club_id": club.away_club_id,
                "date": club.date,
                "time": club.time,
                "location": club.location,
                "status": club.status,
                "home_score": club.home_score,
                "away_score": club.away_score,
            })
        })
        .collect()
}

pub fn get_match_by_id(id: u64) -> Option<Value> {
    repository::get_match_by_id(id).map(|club| {
        let home_club = club_service::get_club_by_id(club.home_club_id);
        let away_club = club_service::get_club_by_id(club.away_club_id);

        serde_json::json!({
            "id": club.id,
            "home_club": home_club,
            "away_club": away_club,
            "date": club.date,
            "time": club.time,
            "location": club.location,
            "status": club.status,
            "home_score": club.home_score,
            "away_score": club.away_score,
        })
    })
}
