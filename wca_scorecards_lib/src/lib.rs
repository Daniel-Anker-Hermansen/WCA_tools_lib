use std::{collections::HashMap, io};

use scorecard_to_pdf::{Competition, Scorecard};

mod blank;
mod parse;
pub mod wcif;

pub use scorecard_to_pdf::{Error as ScorecardsPdfError, Mode};
pub use wca_oauth as oauth;
use wca_oauth::Wcif;

pub enum Output {
	/// A single PDF
	Pdf(Vec<u8>),

	/// A zip files of PDFs
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
	/// An error happened when generating PDFs. This is a bug.
	ScorecardsPdfError(ScorecardsPdfError),

	/// Parsing error when parsing CSV files.
	ParseError(ParseError),

	/// IO error. I think this only happens if it is a bug. // TODO: figure out if this can
	/// happen legitimately
	IoError(io::Error),

	/// Error during zipping. This is a bug.
	ZipError(zip::result::ZipError),

	/// Network error. Try again.
	NetworkError(wca_oauth::NetworkError),

	/// Unable to parse Json received by WCA.
	JsonError(wca_oauth::JsonError),

	/// Error reported by WCA when working with WCIF.
	WcifError(String),
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
		match value {
			zip::result::ZipError::Io(error) => ScorecardsError::IoError(error),
			value => ScorecardsError::ZipError(value),
		}
	}
}

impl From<wca_oauth::Error> for ScorecardsError {
	fn from(value: wca_oauth::Error) -> Self {
		match value {
			wca_oauth::Error::Reqwest(error) => ScorecardsError::NetworkError(error),
			wca_oauth::Error::Json(error) => ScorecardsError::JsonError(error),
			wca_oauth::Error::Wcif(error) => ScorecardsError::WcifError(error),
		}
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
	let (mut scorecards, id_map) = parse::parse_groups_csv(&groups_csv, stages)?;
	let time_limits = parse::parse_limit_csv(limit_csv.as_deref())?;
	let competition = Competition {
		name: competition,
		time_limits: &time_limits,
		id_map: &id_map,
	};
	output_pdf(&mut scorecards, competition, mode, ordering)
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
	scorecards: &mut Vec<Vec<Scorecard>>,
	competition: Competition,
	mode: Mode,
	ordering: ScorecardOrdering,
) -> Result<Output, ScorecardsError> {
	for scorecards in scorecards.iter_mut() {
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
		for (scorecards, stage) in scorecards.iter_mut().zip(1u32..) {
			zip_writer.start_file(
				format!("{}", stage),
				zip::write::FileOptions::<()>::default()
					.compression_method(zip::CompressionMethod::Deflated),
			)?;
			scorecard_to_pdf::generate(scorecards, competition, mode, &mut zip_writer)?;
		}

		drop(zip_writer);
		Ok(Output::Zip(data))
	}
}

pub fn blank_for_subsequent_rounds(
	wcif: &Wcif,
	stations: u32,
	mode: Mode,
	ordering: ScorecardOrdering,
) -> Result<Output, ScorecardsError> {
	let (scorecards, name) = blank::blank_for_subsequent(wcif, stations);
	let competition = Competition {
		name: &name,
		time_limits: &HashMap::new(),
		id_map: &HashMap::new(),
	};
	output_pdf(&mut vec![scorecards], competition, mode, ordering)
}

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
					.then(a.cmp(b))
			}),
		}
	}
}
