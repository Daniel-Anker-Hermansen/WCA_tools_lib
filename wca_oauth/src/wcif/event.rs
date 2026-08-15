use serde::{Deserialize, Serialize};

use super::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Event {
	pub id: String,
	pub rounds: Vec<Round>,
	// pub competitor_limit: Option<usize>, // Not implemented by WCA
	pub qualification: Option<Qualification>,
	pub extensions: Vec<serde_json::Value>,
}
