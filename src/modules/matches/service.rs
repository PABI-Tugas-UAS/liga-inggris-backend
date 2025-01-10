use super::{entity::MatchReq, repository};
use crate::modules::clubs::repository as club_repository;
use serde_json::Value;

pub fn get_matches(req: MatchReq) -> Vec<Value> {
    let matches = repository::get_matches(Some(req));
    matches
        .into_iter()
        .map(|_match| {
            let home_club = club_repository::get_club_by_id(_match.home_club_id);
            let away_club = club_repository::get_club_by_id(_match.away_club_id);

            serde_json::json!({
                "id": _match.id,
                "home_club": home_club,
                "away_club": away_club,
                "date": _match.date,
                "time": _match.time,
                "location": _match.location,
                "status": _match.status,
                "home_score": _match.home_score,
                "away_score": _match.away_score,
            })
        })
        .collect()
}

pub fn get_match_by_id(id: u64) -> Option<Value> {
    repository::get_match_by_id(id).map(|_match| {
        let home_club = club_repository::get_club_by_id(_match.home_club_id);
        let away_club = club_repository::get_club_by_id(_match.away_club_id);
        let match_stats = repository::get_match_stats(_match.id);

        serde_json::json!({
            "id": _match.id,
            "home_club": home_club,
            "away_club": away_club,
            "date": _match.date,
            "time": _match.time,
            "location": _match.location,
            "status": _match.status,
            "home_score": _match.home_score,
            "away_score": _match.away_score,
            "match_stats": match_stats,
        })
    })
}
