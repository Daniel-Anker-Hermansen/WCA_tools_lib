use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResultValue {
	Ok(usize),
	DNF,
	DNS,
	Skip,
}

impl PartialOrd for ResultValue {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for ResultValue {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		let sprio = match self {
			ResultValue::Ok(v) => (0, *v),
			ResultValue::DNF => (1, 0),
			ResultValue::DNS => (1, 1),
			ResultValue::Skip => (1, 2),
		};
		let oprio = match other {
			ResultValue::Ok(v) => (0, *v),
			ResultValue::DNF => (1, 0),
			ResultValue::DNS => (1, 1),
			ResultValue::Skip => (1, 2),
		};
		sprio.cmp(&oprio)
	}
}

impl<'de> Deserialize<'de> for ResultValue {
	fn deserialize<D>(deserializer: D) -> Result<ResultValue, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_i64(AttemptResultVisitor)
	}
}

struct AttemptResultVisitor;

impl<'de> Visitor<'de> for AttemptResultVisitor {
	type Value = ResultValue;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str("an attemptresult")
	}

	fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(match v {
			-1 => ResultValue::DNF,
			-2 => ResultValue::DNS,
			0 => ResultValue::Skip,
			_ => ResultValue::Ok(v as usize),
		})
	}

	fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		Ok(match v {
			0 => ResultValue::Skip,
			_ => ResultValue::Ok(v as usize),
		})
	}
}

impl Serialize for ResultValue {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match self {
			ResultValue::Ok(v) => return v.serialize(serializer),
			_ => (),
		}
		serializer.serialize_i64(match self {
			ResultValue::DNF => -1,
			ResultValue::DNS => -2,
			ResultValue::Skip => 0,
			ResultValue::Ok(v) => *v as i64,
		})
	}
}
