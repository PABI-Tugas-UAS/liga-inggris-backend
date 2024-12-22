use super::entity::{Player, PlayerReq};
use crate::common::utils::{compare::compare_lcase, dummy::read_dummy};

pub fn get_players(req: Option<PlayerReq>) -> Vec<Player> {
    read_dummy::<Vec<Player>>("players/players.json")
        .into_iter()
        .filter(|player| match &req {
            Some(filter) => {
                let matches_club = match filter.club_id {
                    Some(id) => player.club_id == id,
                    None => true,
                };
                let matches_name = match &filter.name {
                    Some(name) => compare_lcase(&player.name, &name),
                    None => true,
                };
                matches_club && matches_name
            }
            None => true,
        })
        .collect()
}

pub fn get_player_by_id(id: u64) -> Option<Player> {
    get_players(None).into_iter().find(|player| player.id == id)
}
