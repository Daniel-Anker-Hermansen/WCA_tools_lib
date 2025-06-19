mod draw_scorecards;
mod font;
mod language;
mod scorecard_generator;
use std::{
	collections::HashMap,
	io::{self, BufWriter},
};

use draw_scorecards::draw_scorecard;
use scorecard_generator::ScorecardGenerator;

pub use printpdf::Error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scorecard<'a> {
	pub event: Option<&'a str>,
	pub round: Option<u32>,
	pub group: Option<u32>,
	pub station: Option<u32>,
	pub id: Option<u32>,
}

fn to_string_or<T: ToString>(val: Option<T>, default: &str) -> String {
	val.as_ref()
		.map(ToString::to_string)
		.unwrap_or(default.to_string())
}

impl Scorecard<'_> {
	pub fn blank<'a>() -> Scorecard<'a> {
		Scorecard {
			event: None,
			round: None,
			group: None,
			station: None,
			id: None,
		}
	}

	pub(crate) fn event(&self) -> String {
		to_string_or(self.event, "")
	}

	pub(crate) fn round(&self) -> String {
		to_string_or(self.round, "__")
	}

	pub(crate) fn group(&self) -> String {
		to_string_or(self.group, "__")
	}

	pub(crate) fn station(&self) -> String {
		to_string_or(self.station, "")
	}

	pub(crate) fn id(&self) -> String {
		to_string_or(self.id, "")
	}

	pub(crate) fn name<'a>(&self, id_map: &HashMap<u32, &'a str>) -> &'a str {
		self.id
			.and_then(|id| id_map.get(&id).copied())
			.unwrap_or("")
	}

	pub(crate) fn limit<'a>(&self, limits: &'a HashMap<&str, TimeLimit>) -> &'a TimeLimit {
		self.event
			.and_then(|event| limits.get(event))
			.unwrap_or(&TimeLimit::None)
	}
}

#[derive(Clone, Copy)]
pub struct Competition<'a> {
	pub name: &'a str,
	pub time_limits: &'a HashMap<&'a str, TimeLimit>,
	pub id_map: &'a HashMap<u32, &'a str>,
}

pub enum TimeLimit {
	Single(usize),
	Cumulative(usize),
	SharedCumulative(usize, Vec<String>),
	Cutoff(usize, usize),
	Multi,
	None,
}

#[non_exhaustive]
#[derive(Clone, Copy)]
pub enum Mode {
	/// A4 size with six scorecards per page.
	A4_6,

	/// A6 size with one scorecard per page.
	A6_1,
}

impl Mode {
	fn number_per_page(&self) -> u32 {
		match self {
			Mode::A4_6 => 6,
			Mode::A6_1 => 1,
		}
	}

	fn page_dimensions(&self) -> (f64, f64) {
		match self {
			Mode::A4_6 => (210.0, 297.0),
			Mode::A6_1 => (105.0, 148.5),
		}
	}

	fn page_layout(&self) -> (u32, u32) {
		match self {
			Mode::A4_6 => (2, 3),
			Mode::A6_1 => (1, 1),
		}
	}
}

/// Writes an encoded PDF to the writer. The scorecards are organised as stated in the mode. It
/// always pads the last page with blank scorecards, so to create a blank scorecard page just give
/// one blank scorecard as argument.
pub fn generate(
	scorecards: &[Scorecard],
	competition: Competition,
	mode: Mode,
	writer: impl io::Write,
) -> Result<(), Error> {
	let mut generator = ScorecardGenerator::new(competition.name, mode);
	let number_of_pages = (scorecards.len() as u32).div_ceil(mode.number_per_page());
	for page in 0..number_of_pages {
		generator.set_page(page);
		for position in 0..mode.number_per_page() {
			let index = number_of_pages * position + page;
			let scorecard = scorecards
				.get(index as usize)
				.copied()
				.unwrap_or(Scorecard::blank());
			generator.set_position(position);
			draw_scorecard(
				&mut generator,
				scorecard,
				competition.id_map,
				competition.time_limits,
			);
		}
	}
	generator.doc().save(&mut BufWriter::new(writer))
}
