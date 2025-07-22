use scorecard_to_pdf::Scorecard;
use wca_oauth::Wcif;

pub(crate) fn blank_for_subsequent(wcif: &Wcif, stations: u32) -> (Vec<Scorecard>, String) {
	let name = wcif.name.clone();
	let mut scorecards = Vec::new();
	for event in &wcif.events {
		let event_name = &event.id;
		let mut prev_round_competitors = wcif
			.persons
			.iter()
			.filter(|p| {
				p.registration
					.as_ref()
					.map(|r| r.event_ids.contains(event_name) && r.status == "accepted")
					.unwrap_or(false)
			})
			.count() as u32;
		for (r, i) in event.rounds.windows(2).zip(2..) {
			let advancement_cond = &r[0].advancement_condition;
			let count =
				crate::wcif::get_max_advancement(prev_round_competitors as usize, advancement_cond)
					as u32;
			let groups = count.div_ceil(stations);
			let count_per_group = count / groups;
			let leftover = count % groups;
			for group in 1..=groups {
				let c = count_per_group + (leftover >= group) as u32;
				for j in 1..=c {
					scorecards.push(Scorecard {
						event: Some(event_name),
						round: Some(i),
						group: Some(group),
						station: Some(j),
						id: None,
					});
				}
			}
			prev_round_competitors = count;
		}
	}

	(scorecards, name)
}
