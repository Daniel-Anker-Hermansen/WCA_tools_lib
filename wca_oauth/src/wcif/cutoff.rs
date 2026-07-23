use serde::{Deserialize, Serialize};

use crate::ResultValue;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Cutoff {
	pub number_of_attempts: usize,
	pub result_value: ResultValue,
}
