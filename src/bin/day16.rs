use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};

use aoc_client::{AocClient, AocResult};
use itertools::Itertools;

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(16)?
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

#[derive(PartialEq, Eq, Debug, Hash, PartialOrd, Ord, Clone, Copy)]
enum Dir {
	Up,
	Down,
	Right,
	Left,
}


fn part1(file: String) -> String {
	let map = file.lines().enumerate().flat_map(|(y, line)| {
		line.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_,_>>();

	let start = map.iter().find(|x| x.1 == &'S').unwrap().0.clone();
	let end = map.iter().find(|x|x.1 == &'E').unwrap().0.clone();

	let mut queue = BinaryHeap::new();
	queue.push((Reverse(0), start, Dir::Right));

	let mut visited = HashSet::new();

	while let Some((Reverse(score), pos, dir)) = queue.pop() {
		let key = (pos, dir);
		if visited.contains(&key) {
			continue;
		}
		visited.insert(key);

		if pos == end {
			return score.to_string();
		}

		let left = match dir {
			Dir::Up => Dir::Left,
			Dir::Down => Dir::Right,
			Dir::Right => Dir::Up,
			Dir::Left => Dir::Down,
		};

		let right = match dir {
			Dir::Up => Dir::Right,
			Dir::Down => Dir::Left,
			Dir::Right => Dir::Down,
			Dir::Left => Dir::Up,
		};

		[
			(score + 1, dir),
			(score + 1001, left),
			(score + 1001, right)
		].into_iter().for_each(|(score, dir)| {
			let next = match dir {
				Dir::Up => (pos.0, pos.1 - 1),
				Dir::Down => (pos.0, pos.1 + 1),
				Dir::Right => (pos.0 + 1, pos.1),
				Dir::Left => (pos.0 - 1, pos.1),
			};

			if !map.contains_key(&next) || map[&next] == '#' {
				return;
			}

			queue.push((Reverse(score), next, dir));
		});
	}

	panic!("not found")
}

fn part2(file: String) -> String {
	let map = file.lines().enumerate().flat_map(|(y, line)| {
		line.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_,_>>();

	let start = map.iter().find(|x| x.1 == &'S').unwrap().0.clone();
	let end = map.iter().find(|x|x.1 == &'E').unwrap().0.clone();

	let mut queue = BinaryHeap::new();
	queue.push((Reverse(0), start, Dir::Right, vec![start]));

	let mut visited = HashMap::new();

	let mut nodes = Vec::new();

	let mut best = Option::None;

	while let Some((Reverse(score), pos, dir, prevs)) = queue.pop() {
		let key = (pos, dir);
		if visited.contains_key(&key) && visited[&key] < score {
			continue;
		}
		visited.insert(key, score);

		if best.is_some() && score > best.unwrap() {
			continue;
		}

		if pos == end {
			best = Some(score);
			nodes.append(&mut prevs.clone());
		}

		let left = match dir {
			Dir::Up => Dir::Left,
			Dir::Down => Dir::Right,
			Dir::Right => Dir::Up,
			Dir::Left => Dir::Down,
		};

		let right = match dir {
			Dir::Up => Dir::Right,
			Dir::Down => Dir::Left,
			Dir::Right => Dir::Down,
			Dir::Left => Dir::Up,
		};

		[
			(score + 1, dir),
			(score + 1001, left),
			(score + 1001, right)
		].into_iter().for_each(|(score, dir)| {
			let next = match dir {
				Dir::Up => (pos.0, pos.1 - 1),
				Dir::Down => (pos.0, pos.1 + 1),
				Dir::Right => (pos.0 + 1, pos.1),
				Dir::Left => (pos.0 - 1, pos.1),
			};

			if !map.contains_key(&next) || map[&next] == '#' {
				return;
			}

			let mut prevs = prevs.clone();
			prevs.push(next);

			queue.push((Reverse(score), next, dir, prevs));
		});
	}

	return nodes.into_iter().unique().count().to_string();
}