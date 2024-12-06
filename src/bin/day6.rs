use std::collections::{HashMap, HashSet};

use aoc_client::{AocClient, AocResult};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Dir {
	Up,
	Down,
	Left,
	Rigth
}

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(6)?
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
	let map = file.lines().enumerate().flat_map(|(y, line)| {
		line.chars().enumerate().map(move |(x, c)| ((y as i32, x as i32), c))
	}).collect::<HashMap<_,_>>();

	let mut curr = *map.iter().find(|x|x.1 == &'^').unwrap().0;

	let mut dir = Dir::Up;
	
	let mut visited = HashSet::new();

	while map.contains_key(&curr) {
		visited.insert(curr);

		let next = match dir {
			Dir::Up => (curr.0 - 1, curr.1),
			Dir::Down => (curr.0 + 1, curr.1),
			Dir::Left => (curr.0, curr.1 - 1),
			Dir::Rigth => (curr.0, curr.1 + 1),
		};

		if map.contains_key(&next) && map[&next] == '#' {
			dir = switch(dir);
		} else {
			curr = next;
		}
	}

	let res = visited.len();
	return res.to_string();
}

fn switch(dir: Dir) -> Dir {
	match dir {
		Dir::Up => Dir::Rigth,
		Dir::Down => Dir::Left,
		Dir::Left => Dir::Up,
		Dir::Rigth => Dir::Down,
	}
}

fn part2(file: String) -> String {
	let map = file.lines().enumerate().flat_map(|(y, line)| {
		line.chars().enumerate().map(move |(x, c)| ((y as i32, x as i32), c))
	}).collect::<HashMap<_,_>>();

	let curr = *map.iter().find(|x|x.1 == &'^').unwrap().0;

	let dir = Dir::Up;

	let mut res = 0;

	for &block in map.keys() {
		if map[&block] == '#' { continue; }
		if cycle(&map, block, curr, dir) {
			res += 1;
		}
	}

	return res.to_string();
}

fn cycle(map: &HashMap<(i32, i32), char>, block: (i32, i32), mut curr: (i32, i32), mut dir: Dir) -> bool {
	let mut visited = HashSet::new();

	while map.contains_key(&curr) {
		let key = (curr, dir);
		if visited.contains(&key) {
			return true;
		}
		visited.insert(key);

		let next = match dir {
			Dir::Up => (curr.0 - 1, curr.1),
			Dir::Down => (curr.0 + 1, curr.1),
			Dir::Left => (curr.0, curr.1 - 1),
			Dir::Rigth => (curr.0, curr.1 + 1),
		};

		if block == next || (map.contains_key(&next) && map[&next] == '#') {
			dir = switch(dir);
		} else {
			curr = next;
		}
	}
	return false;
}