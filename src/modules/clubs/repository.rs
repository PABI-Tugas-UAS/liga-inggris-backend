use super::entity::{Club, ClubReq};
use crate::common::utils::dummy::read_dummy;

pub fn get_clubs(req: Option<ClubReq>) -> Vec<Club> {
    read_dummy::<Vec<Club>>("clubs/clubs.json")
        .into_iter()
        .filter(|club| match &req {
            Some(filter) => match &filter.name {
                Some(name) => club.name.to_lowercase().contains(&name.to_lowercase()),
                None => true,
            },
            None => true,
        })
        .collect()
}

pub fn get_club_by_id(id: u64) -> Option<Club> {
    get_clubs(None).into_iter().find(|club| club.id == id)
}
