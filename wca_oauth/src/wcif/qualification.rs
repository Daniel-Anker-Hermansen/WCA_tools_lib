use serde::{Deserialize, Serialize};

use super::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Qualification {
	pub earliest_result_date: Option<Date>,
	pub latest_result_date: Date,
	pub result_condition: ResultCondition,
}
