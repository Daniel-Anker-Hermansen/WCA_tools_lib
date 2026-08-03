use serde::{Deserialize, Serialize};

use super::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParticipationRuleset {
	pub participation_source: Option<ParticipationSource>,
	pub reserved_places: Option<ReservedPlaces>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ParticipationSource {
	Registrations,
	Round {
		round_id: String,
		result_condition: ResultCondition,
	},
	LinkedRounds {
		round_ids: Vec<String>,
		result_condition: ResultCondition,
	},
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReservedPlaces {
	pub nationalities: Vec<String>,
	pub count: u64,
}
