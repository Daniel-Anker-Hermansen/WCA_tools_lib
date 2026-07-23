use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Series {
	pub id: String,
	pub name: String,
	pub short_name: String,
	pub competitions: Vec<String>,
}
