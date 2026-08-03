use serde::{Deserialize, Serialize};

use crate::ResultValue;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Result {
	pub person_id: usize,
	pub ranking: Option<usize>,
	pub attempts: Vec<Attempt>,
	pub best: ResultValue,
	pub average: ResultValue,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Attempt {
	pub value: ResultValue,
	pub reconstruction: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum ResultCondition {
	ResultAchieved {
		scope: String,
		value: Option<ResultValue>,
	},
	Ranking {
		scope: String,
		value: u64,
	},
	Percent {
		scope: String,
		value: u64,
	},
}
