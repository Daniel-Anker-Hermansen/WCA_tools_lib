use std::{collections::HashMap, io};

use scorecard_to_pdf::{Competition, Scorecard};

mod parse;
mod pdf;
pub mod wcif;

pub use scorecard_to_pdf::{Error as ScorecardsPdfError, Mode};
pub use wca_oauth as oauth;

pub enum Output {
	Pdf(Vec<u8>),
	Zip(Vec<u8>),
}

#[derive(Clone)]
pub struct Stages {
	pub number: u32,
	pub stations_per_stage: u32,
	pub seperate_stages: bool,
}

pub use parse::ParseError;

pub enum ScorecardsError {
	ScorecardsPdfError(ScorecardsPdfError),
	ParseError(ParseError),
	IoError(io::Error),
	ZipError(zip::result::ZipError),
	OauthError(wca_oauth::Error),
}

impl From<ParseError> for ScorecardsError {
	fn from(value: ParseError) -> Self {
		ScorecardsError::ParseError(value)
	}
}

impl From<ScorecardsPdfError> for ScorecardsError {
	fn from(value: ScorecardsPdfError) -> Self {
		match value {
			ScorecardsPdfError::Io(error) => ScorecardsError::IoError(error),
			_ => ScorecardsError::ScorecardsPdfError(value),
		}
	}
}

impl From<io::Error> for ScorecardsError {
	fn from(value: io::Error) -> Self {
		ScorecardsError::IoError(value)
	}
}

impl From<zip::result::ZipError> for ScorecardsError {
	fn from(value: zip::result::ZipError) -> Self {
		ScorecardsError::ZipError(value)
	}
}

impl From<wca_oauth::Error> for ScorecardsError {
	fn from(value: wca_oauth::Error) -> Self {
		ScorecardsError::OauthError(value)
	}
}

pub fn generate_from_csv(
	groups_csv: &str,
	limit_csv: Option<String>,
	competition: &str,
	stages: Stages,
	ordering: ScorecardOrdering,
	mode: Mode,
) -> Result<Output, ScorecardsError> {
	let groups_csv = std::fs::read_to_string(groups_csv)?;
	let limit_csv = limit_csv.map(std::fs::read_to_string).transpose()?;
	let (scorecards, id_map) = parse::parse_groups_csv(&groups_csv, stages)?;
	let time_limits = parse::parse_limit_csv(limit_csv.as_ref().map(String::as_str))?;
	let competition = Competition {
		name: competition,
		time_limits: &time_limits,
		id_map: &id_map,
	};
	output_pdf(scorecards, competition, mode, ordering).map_err(Into::into)
}

pub fn blank_scorecard_page(competition: &str, mode: Mode) -> Result<Output, ScorecardsPdfError> {
	let scorecards = &[Scorecard::blank()];
	let competition = Competition {
		name: competition,
		time_limits: &HashMap::new(),
		id_map: &HashMap::new(),
	};
	let mut data = Vec::new();
	scorecard_to_pdf::generate(scorecards, competition, mode, io::Cursor::new(&mut data))?;
	Ok(Output::Pdf(data))
}

pub fn output_pdf(
	mut scorecards: Vec<Vec<Scorecard>>,
	competition: Competition,
	mode: Mode,
	ordering: ScorecardOrdering,
) -> Result<Output, ScorecardsError> {
	for scorecards in &mut scorecards {
		ordering.sort_slice(scorecards);
	}
	if scorecards.len() == 1 {
		let mut data = Vec::new();
		scorecard_to_pdf::generate(
			&scorecards[0],
			competition,
			mode,
			io::Cursor::new(&mut data),
		)?;
		Ok(Output::Pdf(data))
	} else {
		let mut data = Vec::new();
		let mut zip_writer = zip::ZipWriter::new(io::Cursor::new(&mut data));
		for (scorecards, stage) in scorecards.into_iter().zip(1u32..) {
			zip_writer.start_file(
				format!("{}", stage),
				zip::write::FileOptions::<()>::default()
					.compression_method(zip::CompressionMethod::Deflated),
			)?;
			scorecard_to_pdf::generate(&scorecards, competition, mode, &mut zip_writer)?;
		}

		drop(zip_writer);
		Ok(Output::Zip(data))
	}
}

// TODO: Needs reimplementation
/*
pub fn blank_for_subsequent_rounds(wcif_path: &str, stations: usize) {
	let wcif = std::fs::read_to_string(wcif_path).unwrap();
	let wcif = wca_oauth::parse(wcif).unwrap();
	let data = pdf::blank_for_subsequent(wcif.get(), stations);
	save_pdf(data, &wcif.get().short_name, "").unwrap();
}
*/

#[derive(Clone, Copy)]
pub enum ScorecardOrdering {
	Default,
	ByName,
}

impl ScorecardOrdering {
	fn sort_slice(&self, slice: &mut [Scorecard<'_>]) {
		match self {
			ScorecardOrdering::Default => slice.sort(),
			ScorecardOrdering::ByName => slice.sort_by(|a, b| {
				a.group
					.cmp(&b.group)
					.then(a.station.cmp(&b.station))
					.then(a.id.cmp(&b.id))
					.then(a.cmp(&b))
			}),
		}
	}
}
