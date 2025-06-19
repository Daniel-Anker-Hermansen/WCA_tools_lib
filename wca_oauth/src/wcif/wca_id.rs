use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

#[derive(Debug, PartialEq)]
pub struct WcaId {
	pub year: u16,
	pub chars: [u8; 4],
	pub id: u8,
}

impl<'de> Deserialize<'de> for WcaId {
	fn deserialize<D>(deserializer: D) -> Result<WcaId, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_str(WcaIdVisitor)
	}
}

struct WcaIdVisitor;

impl Visitor<'_> for WcaIdVisitor {
	type Value = WcaId;

	fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
		formatter.write_str(
			"a string consisting of 4 integers followed by 4 charachters follwed by 2 integers",
		)
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		if v.len() != 10 {
			return Err(E::custom("WcaId too short or too long".to_string()));
		}
		let year = &v[0..4];
		let mut chars = [0; 4];
		chars.copy_from_slice(&v.as_bytes()[4..8]);
		let id = &v[8..10];
		let year = year.parse().map_err(|_| {
			E::custom("The first four characters of a WcaId is not numerical".to_string())
		})?;
		let id = id.parse().map_err(|_| {
			E::custom("The last two characters of a WcaId is not numerical".to_string())
		})?;
		Ok(WcaId { year, chars, id })
	}
}

impl Serialize for WcaId {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let str = format!(
			"{:04}{}{:02}",
			self.year,
			self.chars.iter().map(|u| *u as char).collect::<String>(),
			self.id
		);
		serializer.serialize_str(&str)
	}
}
