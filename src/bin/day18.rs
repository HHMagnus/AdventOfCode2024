use std::collections::{HashSet, VecDeque};

use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(18)?
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
	let size = 70;

	let points = file.lines().take(1024).map(|x| {
		let mut s = x.split(",");
		let x = s.next().unwrap().parse::<i32>().unwrap();
		let y = s.next().unwrap().parse::<i32>().unwrap();
		(x, y)
	}).collect::<Vec<_>>();

	for y in 0..=size {
		for x in 0..=size {
			if points.contains(&(x,y)) {
				print!("#");
			} else {
				print!(".");
			}
		}
		println!("");
	}

	let start = (0,0);
	let end = (size, size);

	let mut queue = VecDeque::new();
	queue.push_back((start, 0));

	let mut visited = HashSet::new();

	while let Some((pos, step)) = queue.pop_front() {
		if visited.contains(&pos) {
			continue;
		}
		visited.insert(pos);

		if pos == end {
			return step.to_string();
		}

		[
			(pos.0 - 1, pos.1),
			(pos.0 + 1, pos.1),
			(pos.0, pos.1 - 1),
			(pos.0, pos.1 + 1)
		].into_iter().filter(|&(x, y)| x >= 0 && y >= 0 && x <= size && y <= size).filter(|xy| !points.contains(xy)).for_each(|xs| queue.push_back((xs, step+1)));
	}
	panic!("No solution")
}

fn part2(file: String) -> String {
	let size = 70;

	let points = file.lines().map(|x| {
		let mut s = x.split(",");
		let x = s.next().unwrap().parse::<i32>().unwrap();
		let y = s.next().unwrap().parse::<i32>().unwrap();
		(x, y)
	}).collect::<Vec<_>>();

	for y in 0..=size {
		for x in 0..=size {
			if points.contains(&(x,y)) {
				print!("#");
			} else {
				print!(".");
			}
		}
		println!("");
	}

	let end = (0,0);
	let start = (size, size);

	let mut count = 1024;

	'o: loop {
		let subset = points[0..count].to_vec();
		let mut queue = VecDeque::new();
		queue.push_back((start, 0));
	
		let mut visited = HashSet::new();
	
		while let Some((pos, step)) = queue.pop_front() {
			if visited.contains(&pos) {
				continue;
			}
			visited.insert(pos);
	
			if pos == end {
				count += 1;
				continue 'o;
			}
	
			[
				(pos.0 - 1, pos.1),
				(pos.0 + 1, pos.1),
				(pos.0, pos.1 - 1),
				(pos.0, pos.1 + 1)
			].into_iter().filter(|&(x, y)| x >= 0 && y >= 0 && x <= size && y <= size).filter(|xy| !subset.contains(xy)).for_each(|xs| queue.push_back((xs, step+1)));
		}

		return format!("{},{}", points[count-1].0, points[count-1].1);
	}

	
}