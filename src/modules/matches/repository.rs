use super::entity::{Match, MatchReq};
use crate::common::utils::{compare::compare_lcase, dummy::read_dummy};

pub fn get_matches(req: Option<MatchReq>) -> Vec<Match> {
    read_dummy::<Vec<Match>>("matches/matches.json")
        .into_iter()
        .filter(|_match| match &req {
            Some(filter) => match &filter.status {
                Some(status) => compare_lcase(&_match.status, &status),
                None => true,
            },
            None => true,
        })
        .collect()
}

pub fn get_match_by_id(id: u64) -> Option<Match> {
    get_matches(None).into_iter().find(|_match| _match.id == id)
}
