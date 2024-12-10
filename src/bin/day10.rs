use std::collections::{HashMap, HashSet, VecDeque};

use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(10)?
        .build()?;

    let input: String = client.get_input()?;

    let answer_part1 = part1(input.clone());
	println!("Part 1: {}", answer_part1);
	if answer_part1 == "0" || answer_part1 == "1" { panic!("One or zero???") }
    let part1 = client.submit_answer(1, answer_part1)?;
	println!("Part 1: {:?}", part1);
	
    let answer_part2 = part2(input);
	println!("Part 2: {}", answer_part2);
	if answer_part2 == "0" || answer_part2 == "1" { panic!("One or zero???") }
    let part2 = client.submit_answer(2, answer_part2)?;
	println!("Part 2: {:?}", part2);

	Ok(())
}

fn part1(file: String) -> String {
	let map = file.lines().enumerate().flat_map(|(y,line)| {
		line.chars().map(|x| x.to_digit(10).unwrap()).enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_,_>>();

	let starts = map.iter().filter(|x| x.1 == &0).collect::<Vec<_>>();

	let mut res = 0;

	for start in starts {
		let mut queue = VecDeque::new();
		queue.push_back(*start.0);

		let mut visited = HashSet::new();

		let mut score = 0;

		while let Some(pos) = queue.pop_front() {
			if visited.contains(&pos) {
				continue;
			}
			visited.insert(pos);

			let num = map[&pos];

			if num == 9 {
				score += 1;
				continue;
			}

			let next = num + 1;

			[
				(pos.0 + 1, pos.1),
				(pos.0 - 1, pos.1),
				(pos.0, pos.1 + 1),
				(pos.0, pos.1 - 1),
			].into_iter()
				.filter(|x| map.contains_key(x))
				.filter(|x| map[x] == next)
				.for_each(|x| queue.push_back(x));
		}

		res += score;
	}
	return res.to_string();
}

fn part2(file: String) -> String {
	let map = file.lines().enumerate().flat_map(|(y,line)| {
		line.chars().map(|x| x.to_digit(10).unwrap()).enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_,_>>();

	let starts = map.iter().filter(|x| x.1 == &0).collect::<Vec<_>>();

	let mut res = 0;

	for start in starts {
		let mut queue = VecDeque::new();
		queue.push_back(*start.0);

		let mut score = 0;

		while let Some(pos) = queue.pop_front() {
			let num = map[&pos];

			if num == 9 {
				score += 1;
				continue;
			}

			let next = num + 1;

			[
				(pos.0 + 1, pos.1),
				(pos.0 - 1, pos.1),
				(pos.0, pos.1 + 1),
				(pos.0, pos.1 - 1),
			].into_iter()
				.filter(|x| map.contains_key(x))
				.filter(|x| map[x] == next)
				.for_each(|x| queue.push_back(x));
		}

		res += score;
	}
	return res.to_string();
}