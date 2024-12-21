use std::{collections::{HashMap, HashSet}, vec};

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
	let numerical_map: HashMap<char, (i32, i32)> = [
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
	let directional_map: HashMap<char, (i32, i32)> = [
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
		let shortest = shortest.len() as i32;
		let num = chars.into_iter().filter(|x| x.is_digit(10)).join("").parse::<i32>().unwrap();
		shortest * num
	}).sum::<i32>();

	return res.to_string();
}

fn way(map: &HashMap<char, (i32, i32)>, chars: Vec<char>) -> Vec<Vec<char>> {
	let mut results = Vec::new();
	results.push(Vec::<char>::new());
	for x in chars.windows(2) {
		let x1 = map[&x[0]];
		let x2 = map[&x[1]];

		let presses = presses(map, x1, x2);
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

fn presses(map: &HashMap<char, (i32, i32)>, x1: (i32, i32), x2: (i32, i32)) -> Vec<Vec<char>> {
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

	if !map.values().contains(&outer1) {
		return vec![moves2.iter().chain(moves1.iter()).cloned().collect()];
	}

	if !map.values().contains(&outer2) {
		return vec![moves1.iter().chain(moves2.iter()).cloned().collect()];
	}

	vec![moves2.iter().chain(moves1.iter()).cloned().collect(), moves1.iter().chain(moves2.iter()).cloned().collect()]
}

fn part2(file: String) -> String {
	let numerical_map: HashMap<char, (i32, i32)> = [
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
	let directional_map: HashMap<char, (i32, i32)> = [
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
		let mut chain = way(&numerical_map, chars.clone());
		for i in 0..25 {
			println!("{} {}", i, chain.len());
			chain = chain.into_iter().map(|other| vec!['A'].into_iter().chain(other.into_iter()).collect::<Vec<_>>()).collect::<Vec<_>>();
			chain = chain.into_iter().flat_map(|other| way(&directional_map, other)).collect::<Vec<_>>();
			let min = chain.iter().map(|x| x.len()).min().unwrap();
			let max = chain.iter().map(|x| x.len()).max().unwrap();
			println!("min={}, max={}", min, max);
		}
		let shortest = chain.iter().min_by_key(|x| x.len()).unwrap();
		let shortest = shortest.len() as i32;
		let num = chars.into_iter().filter(|x| x.is_digit(10)).join("").parse::<i32>().unwrap();
		shortest * num
	}).sum::<i32>();

	return res.to_string();
}