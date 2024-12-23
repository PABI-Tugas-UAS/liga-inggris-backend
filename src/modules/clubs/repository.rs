use super::entity::{Club, ClubReq};
use crate::common::utils::{compare::compare_lcase, dummy::read_dummy};

pub fn get_clubs(req: Option<ClubReq>) -> Vec<Club> {
    read_dummy::<Vec<Club>>("clubs/clubs.json")
        .into_iter()
        .filter(|club| match &req {
            Some(filter) => match &filter.name {
                Some(name) => compare_lcase(&club.name, &name),
                None => true,
            },
            None => true,
        })
        .collect()
}

pub fn get_club_by_id(id: u64) -> Option<Club> {
    get_clubs(None).into_iter().find(|club| club.id == id)
}

pub fn get_top_clubs() -> Vec<Club> {
    let mut filtered_clubs: Vec<Club> = get_clubs(None)
        .into_iter()
        .filter(|club| {
            let names = vec!["Manchester United", "Arsenal", "Liverpool"];
            names.iter().any(|name| compare_lcase(&club.name, name))
        })
        .collect();

    if filtered_clubs.len() > 2 {
        let club_at_index_2 = filtered_clubs.remove(2);
        filtered_clubs.insert(0, club_at_index_2);
    }

    filtered_clubs
}
