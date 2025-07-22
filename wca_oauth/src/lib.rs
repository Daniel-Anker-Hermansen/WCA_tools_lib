#![deny(clippy::unwrap_used, clippy::expect_used)]

mod competition;
mod oauth;
mod wcif;
mod wcif_oauth;

pub use competition::*;
pub use oauth::*;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serializer};
pub use wcif::*;
pub use wcif_oauth::*;

pub use chrono::{Datelike, NaiveDate as Date, NaiveDateTime as DateTime, NaiveTime as Time};

const TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";
const DATE_FORMAT: &str = "%Y-%m-%d";

fn de_date_time<'de, D>(deserializer: D) -> std::result::Result<DateTime, D::Error>
where
	D: Deserializer<'de>,
{
	let s: &str = Deserialize::deserialize(deserializer)?;
	DateTime::parse_from_str(s, TIME_FORMAT).map_err(D::Error::custom)
}

fn ser_date_time<S>(date_time: &DateTime, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let str = date_time.format(TIME_FORMAT).to_string();
	serializer.serialize_str(&str)
}

fn de_date<'de, D>(deserializer: D) -> std::result::Result<Date, D::Error>
where
	D: Deserializer<'de>,
{
	let s: &str = Deserialize::deserialize(deserializer)?;
	Date::parse_from_str(s, DATE_FORMAT).map_err(D::Error::custom)
}

fn ser_date<S>(date_time: &Date, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
	S: Serializer,
{
	let str = date_time.format(DATE_FORMAT).to_string();
	serializer.serialize_str(&str)
}

pub enum Error {
	Reqwest(NetworkError),
	Json(serde_json::Error),
	Wcif(String),
}

pub use reqwest::Error as NetworkError;

pub use serde_json::Error as JsonError;

impl From<reqwest::Error> for Error {
	fn from(value: reqwest::Error) -> Self {
		Error::Reqwest(value)
	}
}

impl From<serde_json::Error> for Error {
	fn from(value: serde_json::Error) -> Self {
		Error::Json(value)
	}
}
