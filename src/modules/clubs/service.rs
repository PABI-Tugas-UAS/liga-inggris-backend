use super::{entity::ClubReq, repository};
use crate::modules::clubs::repository as club_repository;
use crate::modules::matches::entity::MatchReq;
use crate::modules::matches::repository as match_repository;
use crate::modules::players::{entity::PlayerReq, repository as player_repository};
use serde_json::Value;

pub fn get_clubs(req: ClubReq) -> Vec<Value> {
    let clubs = repository::get_clubs(Some(req));

    clubs
        .into_iter()
        .map(|club| {
            serde_json::json!({
                "id": club.id,
                "name": club.name,
                "coach": club.coach,
                "founded_year": club.founded_year,
                "achievements": club.achievements,
                "logo": club.logo,
            })
        })
        .collect()
}

pub fn get_club_by_id(id: u64) -> Option<Value> {
    let player_list = player_repository::get_players(Some(PlayerReq {
        club_id: Some(id),
        name: None,
    }));
    let previous_matches = match_repository::get_matches(Some(MatchReq {
        club_id: Some(id),
        status: Some("finished".to_string()),
    }))
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
    .collect::<Vec<Value>>();

    repository::get_club_by_id(id).map(|club| {
        serde_json::json!({
            "id": club.id,
            "name": club.name,
            "coach": club.coach,
            "founded_year": club.founded_year,
            "achievements": club.achievements,
            "logo": club.logo,
            "players": player_list,
            "previous_matches": previous_matches,
        })
    })
}

pub fn get_top_clubs() -> Vec<Value> {
    let clubs = repository::get_top_clubs();

    clubs
        .into_iter()
        .map(|club| {
            serde_json::json!({
                "id": club.id,
                "name": club.name,
                "coach": club.coach,
                "founded_year": club.founded_year,
                "achievements": club.achievements,
                "logo": club.logo,
            })
        })
        .collect()
}
