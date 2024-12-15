use std::collections::HashMap;

use aoc_client::{AocClient, AocResult};
use itertools::Itertools;

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(15)?
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

#[derive(PartialEq, Eq, Debug)]
enum Dir {
	Up,
	Down,
	Right,
	Left,
}

fn part1(file: String) -> String {

	let mut split = file.split("\n\n");

	let mut map = split.next().unwrap().lines().enumerate().flat_map(|(y, line)| {
		line.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_,_>>();

	let instructions = split.next().unwrap().chars().filter_map(|x| {
		match x {
			'v' => Some(Dir::Down),
			'>' => Some(Dir::Right),
			'<' => Some(Dir::Left),
			'^' => Some(Dir::Up),
			_ => None
		}
	}).collect::<Vec<_>>();


	'i: for inst in instructions {
		let robot = map.iter().find(|x| x.1 == &'@').unwrap().0.clone();

		let modifier = match inst {
			Dir::Up => (0, -1),
			Dir::Down => (0, 1),
			Dir::Right => (1, 0),
			Dir::Left => (-1, 0),
		};

		let mut freespace = robot;
		loop {
			freespace = (freespace.0 + modifier.0, freespace.1 + modifier.1);

			if map[&freespace] == '#' {
				continue 'i;
			}

			if map[&freespace] == '.' {
				break;
			}
		}

		while freespace != robot {
			let next = (freespace.0 - modifier.0, freespace.1 - modifier.1);
			let c = map[&next];
			*map.get_mut(&freespace).unwrap() = c;
			freespace = next;
		}

		*map.get_mut(&robot).unwrap() = '.';
	}

	let res = map.iter().filter(|x| x.1 == &'O').map(|x| x.0.0 + x.0.1 * 100).sum::<i32>();

	let maxx = map.iter().map(|x| x.0.0).max().unwrap();
	let maxy = map.iter().map(|x| x.0.1).max().unwrap();
	for y in 0..=maxy {
		for x in 0..=maxx {
			print!("{}", map[&(x, y)]);
		}
		println!("");
	}
	
	return res.to_string();
}

fn part2(file: String) -> String {
	let mut split = file.split("\n\n");

	let mut map = split.next().unwrap().lines().enumerate().flat_map(|(y, line)| {
		line.chars().enumerate().flat_map(move |(x, c)| vec![((x as i32 * 2, y as i32), if c == 'O' { '[' } else { c }), ((x as i32 * 2 + 1, y as i32), if c == '@' { '.' } else if c == 'O' { ']' } else { c })])
	}).collect::<HashMap<_,_>>();

	let instructions = split.next().unwrap().chars().filter_map(|x| {
		match x {
			'v' => Some(Dir::Down),
			'>' => Some(Dir::Right),
			'<' => Some(Dir::Left),
			'^' => Some(Dir::Up),
			_ => None
		}
	}).collect::<Vec<_>>();

	'i: for inst in instructions {
		let robot = map.iter().find(|x| x.1 == &'@').unwrap().0.clone();

		let modifier = match inst {
			Dir::Up => (0, -1),
			Dir::Down => (0, 1),
			Dir::Right => (1, 0),
			Dir::Left => (-1, 0),
		};

		if inst == Dir::Right || inst == Dir::Left {
			let mut freespace = robot;
			loop {
				freespace = (freespace.0 + modifier.0, freespace.1 + modifier.1);

				if map[&freespace] == '#' {
					continue 'i;
				}

				if map[&freespace] == '.' {
					break;
				}
			}

			while freespace != robot {
				let next = (freespace.0 - modifier.0, freespace.1 - modifier.1);
				let c = map[&next];
				*map.get_mut(&freespace).unwrap() = c;
				freespace = next;
			}

			*map.get_mut(&robot).unwrap() = '.';
		} else {

			let mut stack = Vec::new();
			stack.push(vec![robot]);
			
			loop {
				let latest = stack[stack.len()-1]
					.iter()
					.map(|freespace| (freespace.0 + modifier.0, freespace.1 + modifier.1))
					.flat_map(|freespace| {
						if map[&freespace] == '[' {
							vec![freespace, (freespace.0 + 1, freespace.1)]
						} else if map[&freespace] == ']' {
							vec![freespace, (freespace.0 - 1, freespace.1)]
						} else {
							vec![freespace]
						}
					})
					.unique()
					.collect::<Vec<_>>();

				if latest.iter().all(|freespace| map[&freespace] == '.') {
					break;
				}

				if latest.iter().any(|freespace| map[&freespace] == '#' && map[&(freespace.0 - modifier.0, freespace.1 - modifier.1)] != '.') {
					continue 'i;
				}

				let moves = latest.into_iter().filter(|x| map[&x] == '@' || map[&x] == '[' || map[&x] == ']').collect::<Vec<_>>();

				stack.push(moves);
			}

			while let Some(nexts) = stack.pop() {
				for freespace in nexts {
					let next = (freespace.0 + modifier.0, freespace.1 + modifier.1);
					let c = map[&freespace];
					*map.get_mut(&freespace).unwrap() = '.';
					*map.get_mut(&next).unwrap() = c;
				}
			}
		}
	}

	let maxx = map.iter().map(|x| x.0.0).max().unwrap();
	let maxy = map.iter().map(|x| x.0.1).max().unwrap();
	for y in 0..=maxy {
		for x in 0..=maxx {
			print!("{}", map[&(x, y)]);
		}
		println!("");
	}

	let res = map.iter().filter(|x| x.1 == &'[').map(|x| x.0.0 + x.0.1 * 100).sum::<i32>();

	return res.to_string();
}