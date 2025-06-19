use std::collections::HashMap;

use scorecard_to_pdf::{Scorecard, TimeLimit};

use crate::Stages;

pub enum ParseError {
	MissingName(u32),
	MissingId(u32),
	IllegalNumber(u32),
}

// Currently copied spaghetti code. Should be fixed and proper error handling.
pub fn parse_groups_csv(src: &str, stages: Stages) -> Result<(Vec<Vec<Scorecard>>, HashMap<u32, &str>), ParseError> {
	let mut groups_csv = src.lines();
	//Header describing csv file formatting. First two are fixed and therfore skipped.
	//Unwrap cannot fail because the first element of lines always exists, although skip can lead
	//to panic later when used.
	let header = groups_csv.next().unwrap().split(",").skip(2);
	let mut map = HashMap::new();
	let k = groups_csv
		//Filter off empty lines. Fixes annoying EOF issues.
		.filter(|x| *x != "")
		//Map each person to each event they compete in.
		//Enumerate for panic messages
		.enumerate()
		.map(|(line, person)| {
			let mut iter = person.split(",");
			let name = match iter.next() {
				None => panic!("Line {} in csv missing name", line + 2),
				Some(v) => v,
			};
			let id = match iter.next() {
				None => panic!("Line {} in csv missing id", line + 2),
				Some(v) => v,
			};
			let id = match u32::from_str_radix(id, 10) {
				Err(_) => panic!(
					"Id for {} in line {} is not a positive integer",
					name,
					line + 2
				),
				Ok(v) => v,
			};
			//Insert the competitor into the id to name map.
			map.insert(id, name);
			//Zipping with header (clone) to know the order of events.
			iter.zip(header.clone()).filter_map(move |(asign, event)| {
				//Test whether competitor is assigned.
				if asign == "" {
					return None;
				} else {
					let mut info = asign.split(";");
					let pre_group = info.next()?;
					let group = match u32::from_str_radix(pre_group, 10) {
						Err(_) => panic!(
							"Group number for event {} in line {} is nut a positive integer",
							event,
							line + 2
						),
						Ok(v) => v,
					};
					let station = info.next().map(|v| match u32::from_str_radix(v, 10) {
						Err(_) => panic!(
							"Station number for event {} in line {} is not a positive integer",
							event,
							line + 2
						),
						Ok(v) => v,
					});
					let stage = station
						.map(|x| (x as u32 - 1) / stages.stations_per_stage)
						.unwrap_or(0);
					Some((id, event, group, station, stage))
				}
			})
		})
		.flatten()
		.map(|(id, event, group, station, stage)| {
			(
				Scorecard {
					id: Some(id),
					group: Some(group),
					round: Some(1),
					station,
					event: Some(event),
				},
				stage,
			)
		})
		.collect::<Vec<_>>();
	let mut z = Vec::new();
	for (scorecard, stage) in k {
		while z.len() <= stage as usize {
			z.push(Vec::new());
		}
		z[stage as usize].push(scorecard);
	}
	Ok((z, map))
}

pub fn parse_limit_csv(src: Option<&str>) -> Result<HashMap<&str, TimeLimit>, ParseError> {
	match src {
		Some(limit_csv) => {
			//Parse time limits
			let mut limit = limit_csv.lines();
			//Header cannot fail because first in lines
			let event_list = limit.next().unwrap().split(",");
			let limit_data = match limit.next() {
				None => panic!("No time limits given in time limit csv"),
				Some(v) => v,
			}
			.split(",");

			let mut limits = HashMap::new();
			limit_data
				.zip(event_list)
				.try_for_each(|(x, event)| -> Result<(), ParseError> {
					let mut iter = x.split(";");
					let v = match iter.next() {
						None => {
							limits.insert(event, TimeLimit::None);
							return Ok(());
						}
						Some(v) => v,
					};
					match v {
						"T" => limits.insert(event, TimeLimit::Single(usize_from_iter(&mut iter)?)),
						"C" => {
							limits.insert(event, TimeLimit::Cumulative(usize_from_iter(&mut iter)?))
						}
						"K" => limits.insert(
							event,
							TimeLimit::Cutoff(
								usize_from_iter(&mut iter)?,
								usize_from_iter(&mut iter)?,
							),
						),
						"S" => limits.insert(
							event,
							TimeLimit::SharedCumulative(
								usize_from_iter(&mut iter)?,
								iter.map(|x| x.to_string()).collect::<Vec<_>>(),
							),
						),
						"M" => limits.insert(event, TimeLimit::Multi),
						_ => panic!("Malformatted time limit for event: {}", event),
					};
					Ok(())
				});
			Ok(limits)
		}
		None => Ok(HashMap::new()),
	}
}

fn usize_from_iter<'a, I>(iter: &mut I) -> Result<usize, ParseError>
where
	I: Iterator<Item = &'a str>,
{
	iter.next()
		.map(str::parse)
		.map(Result::ok)
		.flatten()
		.ok_or(ParseError::IllegalNumber(0))
}
