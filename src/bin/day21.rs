use std::{collections::HashMap, vec};

use aoc_client::{AocClient, AocResult};
use itertools::Itertools;

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(21)?
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
	let numerical_map: HashMap<char, (i64, i64)> = [
		('A', (0, 0)),
		('0', (-1, 0)),
		('1', (-2, -1)),
		('2', (-1, -1)),
		('3', (0, -1)),
		('4', (-2, -2)),
		('5', (-1, -2)),
		('6', (0, -2)),
		('7', (-2, -3)),
		('8', (-1, -3)),
		('9', (0, -3)),
	].into_iter().collect::<HashMap<_,_>>();
	let directional_map: HashMap<char, (i64, i64)> = [
		('A', (0, 0)),
		('^', (-1, 0)),
		('>', (0, 1)),
		('v', (-1, 1)),
		('<', (-2, 1)),
	].into_iter().collect::<HashMap<_,_>>();
	let lines = file.lines().collect::<Vec<_>>();

	let res = lines.into_iter().map(|x| {
		let chars = x.chars().collect::<Vec<_>>();
		let chars = vec!['A'].into_iter().chain(chars.into_iter()).collect::<Vec<_>>();
		let robot1 = way(&numerical_map, chars.clone());
		let robot1 = robot1.into_iter().map(|other| vec!['A'].into_iter().chain(other.into_iter()).collect::<Vec<_>>()).collect::<Vec<_>>();
		let robot2 = robot1.into_iter().flat_map(|other| way(&directional_map, other)).collect::<Vec<_>>();
		let robot2 = robot2.into_iter().map(|other| vec!['A'].into_iter().chain(other.into_iter()).collect::<Vec<_>>()).collect::<Vec<_>>();
		let human = robot2.into_iter().flat_map(|other| way(&directional_map, other)).collect::<Vec<_>>();
		let shortest = human.iter().min_by_key(|x| x.len()).unwrap();
		let shortest = shortest.len() as i64;
		let num = chars.into_iter().filter(|x| x.is_digit(10)).join("").parse::<i64>().unwrap();
		shortest * num
	}).sum::<i64>();

	return res.to_string();
}

fn way(map: &HashMap<char, (i64, i64)>, chars: Vec<char>) -> Vec<Vec<char>> {
	let mut results = Vec::new();
	results.push(Vec::<char>::new());
	for x in chars.windows(2) {
		let x1 = map[&x[0]];
		let x2 = map[&x[1]];

		let presses = presses(x1, x2);
		results = results.into_iter().flat_map(|vec| {
			if presses.is_empty() {
				let mut cloned = vec.clone();
				cloned.push('A');
				return vec![cloned];
			}
			presses.iter().map(move |xs| {
				let mut cloned = vec.clone();
				cloned.append(&mut xs.clone());
				cloned.push('A');
				cloned
			}).collect::<Vec<_>>()
		}).collect();
	}
	results
}

fn presses(x1: (i64, i64), x2: (i64, i64)) -> Vec<Vec<char>> {
	let diff1 = x1.0 - x2.0;
	let diff2 = x1.1 - x2.1;
	if diff1 == 0 && diff2 == 0 {
		return vec![];
	}
	let char1 = if diff1 > 0 {
		'<'
	} else {
		'>'
	};
	let char2 = if diff2 > 0 {
		'^'
	} else {
		'v'
	};
	let moves1 = (0..diff1.abs()).map(|_| char1).collect::<Vec<_>>();
	let moves2 = (0..diff2.abs()).map(|_| char2).collect::<Vec<_>>();
	if diff2 == 0 {
		return vec![moves1];
	}
	if diff1 == 0 {
		return vec![moves2];
	}
	
	let outer1 = (x2.0, x1.1);
	let outer2 = (x1.0, x2.1);

	let illegal_position = (-2, 0);

	if illegal_position == outer1 {
		return vec![moves2.iter().chain(moves1.iter()).cloned().collect()];
	}

	if illegal_position == outer2 {
		return vec![moves1.iter().chain(moves2.iter()).cloned().collect()];
	}

	vec![moves2.iter().chain(moves1.iter()).cloned().collect(), moves1.iter().chain(moves2.iter()).cloned().collect()]
}

fn best_way(cache: &mut HashMap<(Vec<char>, (i64, i64), i64), i64>, map: &HashMap<char, (i64, i64)>, sequence: Vec<char>, depth: i64, curr: (i64, i64)) -> i64 {
	if sequence.is_empty() {
		return 0;
	}

	let cache_key = (sequence.clone(), curr.clone(), depth.clone());
	if cache.contains_key(&cache_key) {
		return cache[&cache_key];
	}
	
	let next = sequence[0];
	let next_pos = map[&next];

	let presses = presses(curr, next_pos);

	let min = if depth == 0 {
		presses.into_iter().map(|x| x.len() as i64).min().unwrap_or(0) + 1
	} else {
		presses.into_iter()
		.map(|mut x| {
			x.push('A');
			best_way(cache, map, x, depth-1, (0, 0))
		})
		.min()
		.unwrap_or(1)
	};

	let res = min + best_way(cache, map, sequence[1..].to_vec(), depth, next_pos);
	cache.insert(cache_key, res);
	return res;
}

fn part2(file: String) -> String {
	let numerical_map: HashMap<char, (i64, i64)> = [
		('A', (0, 0)),
		('0', (-1, 0)),
		('1', (-2, -1)),
		('2', (-1, -1)),
		('3', (0, -1)),
		('4', (-2, -2)),
		('5', (-1, -2)),
		('6', (0, -2)),
		('7', (-2, -3)),
		('8', (-1, -3)),
		('9', (0, -3)),
	].into_iter().collect::<HashMap<_,_>>();
	let directional_map: HashMap<char, (i64, i64)> = [
		('A', (0, 0)),
		('^', (-1, 0)),
		('>', (0, 1)),
		('v', (-1, 1)),
		('<', (-2, 1)),
	].into_iter().collect::<HashMap<_,_>>();
	let lines = file.lines().collect::<Vec<_>>();

	let res = lines.into_iter().map(|x| {
		let chars = x.chars().collect::<Vec<_>>();
		let chars = vec!['A'].into_iter().chain(chars.into_iter()).collect::<Vec<_>>();
		let chain = way(&numerical_map, chars.clone());
		let shortest = chain.into_iter().map(|x| best_way(&mut HashMap::new(), &directional_map, x, 24, (0, 0))).min().unwrap();
		let num = chars.into_iter().filter(|x| x.is_digit(10)).join("").parse::<i64>().unwrap();
		println!("{}: {} * {}", x, shortest, num);
		shortest * num
	}).sum::<i64>();

	return res.to_string();
}