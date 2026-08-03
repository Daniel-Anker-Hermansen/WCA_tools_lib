use serde::{Deserialize, Serialize};

use super::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Round {
	pub id: String,
	pub linked_rounds: Option<Vec<String>>,
	pub format: char,
	pub time_limit: Option<TimeLimit>,
	pub cutoff: Option<Cutoff>,
	pub participation_ruleset: Option<ParticipationRuleset>,
	pub results: Vec<Result>,
	pub scramble_set_count: usize,
	// pub scramble_sets: Vec<ScrambleSet>, // scramble_sets is not implemented by the wca
	// currerntly.
	pub extensions: Vec<serde_json::Value>,
}
