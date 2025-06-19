use std::collections::HashMap;
use crate::ScorecardOrdering;
use crate::wcif::get_round_json;
use scorecard_to_pdf::{Scorecard, TimeLimit};
use wca_oauth::{WcifContainer, Wcif};

pub(crate) fn run_from_wcif(wcif: &mut WcifContainer, event: &str, round: usize, groups: Vec<Vec<(usize, usize)>>, stages: &Stages, compare: ScorecardOrdering) -> Return {
    let (map, limit, competition) = crate::wcif::get_scorecard_info_for_round(wcif, event, round);

    //Unwrap should not fail as the existence of this round is already confirmed at this point.
    get_round_json(wcif, event, round).unwrap().scramble_set_count = groups.len();
    let mut limits = HashMap::new();
    limits.insert(event, limit);

    let mut k = groups.into_iter()
        .zip(1..)
        .map(|(group, n)|{
            group.into_iter()
                .map(move |(id, station)|{
                    Scorecard {
                        event,
                        round,
                        group: if stages.seperate_stages { (n - 1) * stages.no as usize + 1 + station / stages.capacity as usize } else { n },
                        station: Some(if stages.seperate_stages { (station -  1) % stages.capacity as usize  + 1 } else { station }),
                        id: Some(id),
                        stage: Some((station as u32 - 1) / stages.capacity),
                    }
                })
        }).flatten()
        .collect::<Vec<_>>();

    compare.sort_slice(&mut k);
    
    scorecards_to_pdf(k, &competition, &map, &limits, Language::english())
}


pub(crate) fn blank_for_subsequent(wcif: &Wcif, stations: usize) -> Return {
    let name = &wcif.name;
    let mut scorecards = Vec::new();
    for event in &wcif.events {
        let event_name = &event.id;
        let mut prev_round_competitors = wcif.persons.iter().filter(|p| p.registration.as_ref().map(|r| r.event_ids.contains(event_name) && r.status == "accepted").unwrap_or(false)).count();
        for (i, r) in event.rounds.windows(2).enumerate() {
            let advancement_cond = &r[0].advancement_condition;
            let count = crate::wcif::get_max_advancement(prev_round_competitors, advancement_cond);
            let groups = count.div_ceil(stations);
            let count_per_group = count / groups;
            let leftover = count % groups;
            for group in 1..=groups {
                let c = count_per_group + (leftover >= group) as usize;
                for j in 1..=c {
                    scorecards.push(Scorecard {
                        event: event_name,
                        round: i + 2,
                        group,
                        station: Some(j),
                        id: None,
                        stage: None,
                    });
                }
            }
            prev_round_competitors = count;
        }
    }

    scorecard_to_pdf::scorecards_to_pdf(scorecards, name, &HashMap::new(), &HashMap::new(), Language::english())
}
