use scorecard_to_pdf::Return;
use wca_oauth::{Assignment, AssignmentCode, WcifOAuth};

use crate::{Stages, ScorecardOrdering};

pub async fn generate_pdf(eventid: &str, round: usize, groups: Vec<Vec<usize>>, wcif: bool, wcif_oauth: &mut WcifOAuth, stages: &Stages, compare: ScorecardOrdering) -> Return {
    fn assign_stages(groups: Vec<Vec<usize>>, stages: &Stages) -> Vec<Vec<(usize, usize)>> {
        groups.into_iter()
            .map(|group| {
                    let no_of_stages = (group.len() + stages.capacity as usize - 1) / stages.capacity as usize;
                    (0..no_of_stages).cycle().zip(group).enumerate()
                        .map(|(idx, (i, g))| {
                            let station = stages.capacity as usize * i + idx / no_of_stages + 1;
                            (g, station)
                        })
                        .collect()
                })
            .collect()
    }
    
    let groups_with_stations = assign_stages(groups.clone(), stages);

    if wcif {
        match wcif_oauth.add_groups_to_event(eventid, round, groups.len()) {
            Ok(activities) => {
                let mut activity_ids: Vec<_> = activities.into_iter().map(|act| act.id).collect();
                // Add the first activity id to the end so that we can use array_windows.
                activity_ids.extend_from_within(..1);
                for (group, [activity_id, next_activity_id]) in groups_with_stations.iter()
                    .zip(activity_ids.array_windows().cloned())
                {
                    for (id, station) in group.into_iter() {
                        //This runs in O(nm) time which is horrible, when it could run in O(n) time but n and m are both small so i will let it be for now :)
                        wcif_oauth.patch_persons(|person|{
                            if person.registrant_id == Some(*id) {
                                // Push competing assignet to current group
                                person.assignments.push(Assignment { activity_id, assignment_code: AssignmentCode::Competitor, station_number: Some(*station) });
                                if activity_id != next_activity_id {
                                    // Push judge assignment to next group
                                    person.assignments.push(Assignment { activity_id: next_activity_id, assignment_code: AssignmentCode::Judge, station_number: None });
                                }
                            }
                        });
                    }
                }
                let response = wcif_oauth.patch().await;
                println!("Patched to wcif. Received the following response: \n{}", response);
            }
            Err(()) => println!("Unable to patch likely because the given event already has groups in the wcif."),
        }
    }

    crate::pdf::run_from_wcif(wcif_oauth, eventid, round, groups_with_stations, &stages, compare)
}
