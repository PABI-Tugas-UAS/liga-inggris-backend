use super::entity::{Match, MatchReq};
use crate::common::utils::dummy::read_dummy;

pub fn get_matches(req: Option<MatchReq>) -> Vec<Match> {
    read_dummy::<Vec<Match>>("matches/matches.json")
        .into_iter()
        .filter(|_match| match &req {
            Some(filter) => match &filter.status {
                Some(status) => _match
                    .status
                    .to_lowercase()
                    .contains(&status.to_lowercase()),
                None => true,
            },
            None => true,
        })
        .collect()
}

pub fn get_match_by_id(id: u64) -> Option<Match> {
    get_matches(None).into_iter().find(|_match| _match.id == id)
}
