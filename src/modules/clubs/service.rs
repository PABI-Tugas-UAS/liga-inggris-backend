use super::{entity::ClubReq, repository};
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
    repository::get_club_by_id(id).map(|club| {
        serde_json::json!({
            "id": club.id,
            "name": club.name,
            "coach": club.coach,
            "founded_year": club.founded_year,
            "achievements": club.achievements,
            "logo": club.logo,
        })
    })
}
