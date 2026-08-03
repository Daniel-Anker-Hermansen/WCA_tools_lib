use std::collections::HashMap;

use wca_oauth::*;

use scorecard_to_pdf::TimeLimit;

pub fn get_rounds(wcif: &mut WcifContainer) -> Vec<(String, usize)> {
	wcif.events_iter()
		.flat_map(|event| event.rounds.iter().map(|round| round.id.to_string()))
		.map(|str| {
			let mut iter = str.split("-r");
			(
				iter.next().unwrap().to_string(),
				iter.next().unwrap().parse().unwrap(),
			)
		})
		.collect()
}

pub fn get_scorecard_info_for_round(
	wcif: &mut WcifContainer,
	event: &str,
	round: usize,
) -> (HashMap<usize, String>, TimeLimit, String) {
	let id_map = get_id_map(wcif);
	let time_limit = get_time_limit(wcif, event, round);
	(id_map, time_limit, wcif.get().name.clone())
}

pub fn get_time_limit(wcif: &mut WcifContainer, event: &str, round: usize) -> TimeLimit {
	let round_json = get_round_json(wcif, event, round).unwrap();
	match &round_json.time_limit {
		None => TimeLimit::Multi,
		Some(v) => match round_json.cutoff {
			None => match v.cumulative_round_ids.len() {
				0 => TimeLimit::Single(v.centiseconds),
				1 => TimeLimit::Cumulative(v.centiseconds),
				_ => TimeLimit::SharedCumulative(v.centiseconds, v.cumulative_round_ids.clone()),
			},
			Some(ref c) => TimeLimit::Cutoff(
				v.centiseconds,
				if let ResultValue::Ok(v) = c.result_value {
					v
				} else {
					unreachable!()
				},
			),
		},
	}
}

pub fn wca_live_get_competitors_for_round(
	wcif: &mut WcifContainer,
	event: &str,
	round: usize,
) -> (Vec<usize>, HashMap<usize, String>) {
	let id_map = get_id_map(wcif);
	// Get the previous round, so we can sort people correctly by speed.
	let round_json_prev = get_round_json(wcif, event, round - 1);
	let advancement_ids_prev: HashMap<_, _> = match round_json_prev {
		Some(v) => v
			.results
			.iter()
			.map(|r| {
				(
					r.person_id,
					r.ranking
						.expect("This is a previous round, so there is a ranking"),
				)
			})
			.collect(),
		None => HashMap::new(),
	};

	// Now actually get those who proceeded
	let round_json = get_round_json(wcif, event, round).expect("Round should exist");
	let mut advancement_ids = wca_live_get_advancement_ids(round_json);
	if !advancement_ids.is_empty() {
		if !advancement_ids_prev.is_empty() {
			advancement_ids.sort_by_key(|&x| advancement_ids_prev.get(&x).unwrap_or(&usize::MAX));
			(advancement_ids, id_map)
		} else {
			(advancement_ids, id_map)
		}
	} else {
		get_competitors_for_round(wcif, event, round)
	}
}

pub fn get_competitors_for_round(
	wcif: &mut WcifContainer,
	event: &str,
	round: usize,
) -> (Vec<usize>, HashMap<usize, String>) {
	let id_map = get_id_map(wcif);
	(
		get_participation(wcif.get(), event, round as u64).expect("The round exists"),
		id_map,
	)
}

pub(crate) fn get_round_json<'a>(
	wcif: &'a mut WcifContainer,
	event: &str,
	round: usize,
) -> Option<&'a mut Round> {
	let activity_id = format!("{}-r{}", event, round);
	wcif.round_iter_mut().find(|round| round.id == activity_id)
}

pub fn get_id_map(wcif: &WcifContainer) -> HashMap<usize, String> {
	wcif.persons_iter()
		.filter_map(|p| p.registrant_id.map(|v| (v, p.name.clone())))
		.collect()
}

fn wca_live_get_advancement_ids(round: &Round) -> Vec<usize> {
	let advacenment_ids = round.results.iter().map(|f| f.person_id).collect();
	advacenment_ids
}

fn get_advancement(
	event: &Event,
	round_ids: &[String],
	result_condition: &ResultCondition,
) -> Option<Vec<usize>> {
	let mut best_results = HashMap::new();
	let single = match result_condition {
		ResultCondition::ResultAchieved { scope, .. } => scope == "single",
		ResultCondition::Ranking { scope, .. } => scope == "single",
		ResultCondition::Percent { scope, .. } => scope == "single",
	};
	for round_id in round_ids {
		let round = event.rounds.iter().find(|round| &round.id == round_id)?;
		for result in &round.results {
			if (!single && matches!(result.average, ResultValue::Ok(_)))
				|| (single && matches!(result.best, ResultValue::Ok(_)))
			{
				let act_average = if single {
					ResultValue::Skip
				} else {
					result.average
				};
				let entry = best_results
					.entry(result.person_id)
					.or_insert((act_average, result.best));
				*entry = (*entry).min((act_average, result.best));
			}
		}
	}
	let mut ids: Vec<_> = best_results.into_iter().collect();
	ids.sort();
	let max_advancement = (ids.len() * 75) / 100;
	let num_advanced = match result_condition {
		ResultCondition::ResultAchieved { value, .. } => {
			if single {
				ids.iter()
					.filter(|(_, (_, single))| match value {
						Some(value) => single < value,
						None => true,
					})
					.count()
			} else {
				ids.iter()
					.filter(|(_, (average, _))| match value {
						Some(value) => average < value,
						None => true,
					})
					.count()
			}
		}
		ResultCondition::Ranking { value, .. } => *value as usize,
		ResultCondition::Percent { value, .. } => (ids.len() * *value as usize) / 100,
	}
	.min(max_advancement);
	let cut = ids[num_advanced].1;
	Some(
		ids.into_iter()
			.filter(|(_, value)| *value < cut)
			.map(|(id, _)| id)
			.collect(),
	)
}

pub(crate) fn get_registered_competitors(wcif: &Wcif, event_id: &str) -> Vec<usize> {
	wcif.persons
		.iter()
		.filter(|person| {
			person
				.registration
				.as_ref()
				.map(|registration| {
					registration.is_competing
						&& registration.status == "accepted"
						&& registration.event_ids.iter().any(|event| event == event_id)
				})
				.unwrap_or(false)
		})
		.filter_map(|person| person.registrant_id)
		.collect()
}

/// Gets all ids that may compete in the round in seeding order. The participation source's
/// advancement condition should be used to determine the actual advancement.
fn get_participation(wcif: &Wcif, event_id: &str, round: u64) -> Option<Vec<usize>> {
	let event = wcif.events.iter().find(|event| event.id == event_id)?;
	let round = event.rounds.get((round - 1) as usize)?;
	let participation_source = round
		.participation_ruleset
		.as_ref()?
		.pariticipation_source
		.as_ref()?;
	dbg!("here");

	match participation_source {
		ParticipationSource::Registrations => Some(get_registered_competitors(wcif, event_id)),
		ParticipationSource::Round {
			round_id,
			result_condition,
		} => get_advancement(event, core::slice::from_ref(round_id), result_condition),
		ParticipationSource::LinkedRounds {
			round_ids,
			result_condition,
		} => get_advancement(event, round_ids, result_condition),
	}
}
