use std::collections::{HashMap, HashSet};
use itertools::Itertools;

use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(8)?
        .build()?;

    let input: String = client.get_input()?;

    let answer_part1 = part1(input.clone());
    let part1 = client.submit_answer(1, answer_part1)?;
	println!("{:?}", part1);
	
    let answer_part2 = part2(input);
    let part2 = client.submit_answer(2, answer_part2)?;
	println!("{:?}", part2);

	Ok(())
}

fn part1(file: String) -> String {
	let lines = file.lines().enumerate().flat_map(|(y, line)| {
		line.chars().into_iter().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_,_>>();

	let unique = lines.values().into_iter().cloned().unique().collect::<Vec<_>>();

	let mut antinodes = HashSet::new();

	for un in unique {
		if un == '.' {
			continue;
		}

		let diffs = lines.iter().filter(|x| *x.1 == un).map(|x| *x.0).collect::<Vec<_>>();

		if diffs.len() == 1 {
			continue;
		}

		for i in 0..diffs.len() {
			for j in i+1..diffs.len() {
				let diff1 = diffs[i];
				let diff2 = diffs[j];

				antinodes.insert((diff1.0 + (diff1.0 - diff2.0), diff1.1 + (diff1.1 - diff2.1)));
				antinodes.insert((diff2.0 + (diff2.0 - diff1.0), diff2.1 + (diff2.1 - diff1.1)));
			}
		}
	}

	let res = antinodes.into_iter().filter(|x| lines.contains_key(x)).count();
	return res.to_string();
}

fn part2(file: String) -> String {
	let lines = file.lines().enumerate().flat_map(|(y, line)| {
		line.chars().into_iter().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_,_>>();

	let unique = lines.values().into_iter().cloned().unique().collect::<Vec<_>>();

	let mut antinodes = HashSet::new();

	for un in unique {
		if un == '.' {
			continue;
		}

		let diffs = lines.iter().filter(|x| *x.1 == un).map(|x| *x.0).collect::<Vec<_>>();

		if diffs.len() == 1 {
			continue;
		}

		diffs.iter().cloned().for_each(|x| { antinodes.insert(x); });

		for i in 0..diffs.len() {
			for j in i+1..diffs.len() {
				let diff1 = diffs[i];
				let diff2 = diffs[j];

				let mut next = (diff1.0 + (diff1.0 - diff2.0), diff1.1 + (diff1.1 - diff2.1));
				while lines.contains_key(&next) {
					antinodes.insert(next.clone());

					next = (next.0 + (diff1.0 - diff2.0), next.1 + (diff1.1 - diff2.1))
				}

				let mut next = (diff2.0 + (diff2.0 - diff1.0), diff2.1 + (diff2.1 - diff1.1));
				while lines.contains_key(&next) {
					antinodes.insert(next.clone());

					next = (next.0 + (diff2.0 - diff1.0), next.1 + (diff2.1 - diff1.1))
				}
			}
		}
	}

	let res = antinodes.into_iter().count();
	return res.to_string();
}