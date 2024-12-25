use std::collections::HashMap;

use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(25)?
        .build()?;

    let input: String = client.get_input()?;

    let answer_part1 = part1(input.clone());
	println!("Part 1: {}", answer_part1);
	if answer_part1 == "0" || answer_part1 == "1" { panic!("One or zero???") }
    let part1 = client.submit_answer(1, answer_part1)?;
	println!("Part 1: {:?}", part1);

	Ok(())
}

fn part1(file: String) -> String {

	let different = file.split("\n\n").map(|one| {
		one.lines().enumerate().flat_map(|(y, cs)| {
			cs.chars().enumerate().map(move |(x, c)| ((x, y), c))
		}).collect::<HashMap<_,_>>()
	}).collect::<Vec<_>>();


	let locks = different.iter().cloned().filter(|map| map.iter().any(|((_, y), c)| y == &0 && c == &'#')).collect::<Vec<_>>();
	let keys = different.iter().cloned().filter(|map| map.iter().all(|((_, y), c)| !(y == &0 && c == &'#'))).collect::<Vec<_>>();
	let mut res = 0;

	for key in keys {
		let key = heights(&key);
		'l: for lock in &locks {
			let lock = heights(lock);
			for i in 0..5 {
				if key[i] + lock[i] > 7 {
					continue 'l;
				}
			}
			res += 1;
		}
	}
	return res.to_string();
}

fn heights(map: &HashMap<(usize, usize), char>) -> Vec<usize> {
	let mut res = Vec::new();
	for x in 0..6 {
		let mut counter = 0;
		for y in 0..=7 {
			let key = (x, y);
			if map.contains_key(&key) && map[&key] == '#' {
				counter += 1;
			}
		}
		res.push(counter);
	}

	res
}