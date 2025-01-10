use super::{entity::PlayerReq, repository};
use crate::modules::clubs::repository as club_repository;
use serde_json::Value;

pub fn get_players(req: PlayerReq) -> Vec<Value> {
    let players = repository::get_players(Some(req));

    players
        .into_iter()
        .map(|player| {
            serde_json::json!({
                "id": player.id,
                "club_id": player.club_id,
                "name": player.name,
                "position": player.position,
                "age": player.age,
            })
        })
        .collect()
}

pub fn get_player_by_id(id: u64) -> Option<Value> {
    let player = repository::get_player_by_id(id)?;
    let club = club_repository::get_club_by_id(player.club_id);

    Some(serde_json::json!({
        "id": player.id,
        "club": club,
        "name": player.name,
        "position": player.position,
        "age": player.age,
    }))
}
