use serde::{Deserialize, Serialize};
use super::*;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationInfo {
	#[serde(
		deserialize_with = "crate::de_date_time",
		serialize_with = "crate::ser_date_time"
	)]
	pub open_time: DateTime,
	#[serde(
		deserialize_with = "crate::de_date_time",
		serialize_with = "crate::ser_date_time"
	)]
	pub close_time: DateTime,
	pub base_entry_fee: u64,
	pub currency_code: String,
	pub on_the_spot_registration: bool,
	pub use_wca_registration: bool,
}
