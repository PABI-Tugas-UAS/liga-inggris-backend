use super::entity::{Player, PlayerReq};
use crate::common::utils::dummy::read_dummy;

pub fn get_players(req: Option<PlayerReq>) -> Vec<Player> {
    read_dummy::<Vec<Player>>("players/players.json")
        .into_iter()
        .filter(|player| match &req {
            Some(filter) => match filter.club_id {
                Some(id) => player.club_id == id,
                None => true,
            },
            None => true,
        })
        .collect()
}

pub fn get_player_by_id(id: u64) -> Option<Player> {
    get_players(None).into_iter().find(|player| player.id == id)
}
