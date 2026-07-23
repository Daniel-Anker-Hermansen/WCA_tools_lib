use serde::{Deserialize, Serialize};

mod activity;
mod advancement_condition;
mod assignment;
mod avatar;
mod cutoff;
mod event;
mod person;
mod personal_best;
mod qualification;
mod registration;
mod registration_info;
mod result;
mod result_value;
mod role;
mod room;
mod round;
mod schedule;
mod series;
mod time_limit;
mod venue;
mod wca_id;
mod pariticipation_ruleset;
mod scramble_set;

pub use super::{Date, DateTime};
pub use activity::*;
pub use advancement_condition::*;
pub use assignment::*;
pub use avatar::*;
pub use cutoff::*;
pub use event::*;
pub use person::*;
pub use personal_best::*;
pub use qualification::*;
pub use registration::*;
pub use registration_info::*;
pub use result::*;
pub use result_value::*;
pub use role::*;
pub use room::*;
pub use round::*;
pub use schedule::*;
pub use series::*;
pub use time_limit::*;
pub use venue::*;
pub use wca_id::*;
pub use pariticipation_ruleset::*;
pub use scramble_set::*;

use crate::WcifContainer;

pub type WcifResult = std::result::Result<WcifContainer, WcifError>;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Wcif {
	pub format_version: String,
	pub id: String,
	pub name: String,
	pub short_name: String,
	pub series: Option<Series>,
	pub persons: Vec<Person>,
	pub events: Vec<Event>,
	pub schedule: Schedule,
	pub registration_info: RegistrationInfo,
	pub competitor_limit: Option<usize>,
	pub extensions: Vec<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct WcifError {
	pub error: String,
}

pub fn parse(json: String) -> WcifResult {
	serde_json::from_str(&json)
		.map(WcifContainer::new)
		.map_err(|_| serde_json::from_str(&json).unwrap())
}
