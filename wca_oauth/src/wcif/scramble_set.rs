use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScrambleSet {
	id: u64,
	scrambles: Vec<String>,
	extra_scrambles: Vec<String>,
}
