use std::collections::HashMap;

use scorecard_to_pdf::{generate, Competition, Mode, Scorecard};

fn main() {
	let scorecards: [_; 7] = std::array::from_fn(|i| Scorecard {
		event: Some("3x3x3 Team Blind"),
		round: Some(1),
		group: None,
		station: Some(i as _),
		id: Some(1),
	});
	let competition = Competition {
		name: "This is a test",
		time_limits: &HashMap::new(),
		id_map: &HashMap::from_iter([(1, "Arya Stark")]),
	};
	let mut file = std::fs::File::create("test.pdf").unwrap();
	generate(&scorecards, competition, Mode::A6_1, &mut file).unwrap();
}
