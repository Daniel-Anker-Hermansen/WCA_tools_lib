use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Competition {
	pub id: String,
	pub name: String,
	pub registration_open: String,
	pub registration_close: String,
	pub announced_at: Option<String>,
	pub start_date: String,
	pub end_date: String,
	pub competitor_limit: Option<u64>,
	pub cancelled_at: Option<String>,
	pub url: Option<String>,
	pub website: Option<String>,
	pub short_name: String,
	pub city: String,
	pub venue_address: Option<String>,
	pub venue_details: Option<String>,
	pub latitude_degrees: f64,
	pub longitude_degrees: f64,
	pub country_iso2: String,
	pub event_ids: Vec<String>,
	pub delegates: Vec<serde_json::Value>,
	pub organizers: Vec<serde_json::Value>,
}

impl Competition {
	pub fn from_json(json: &str) -> core::result::Result<Vec<Competition>, Error> {
		serde_json::from_str(json).map_err(Into::into)
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn id(&self) -> &str {
		&self.id
	}
}
