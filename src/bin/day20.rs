use std::collections::{HashMap, VecDeque};

use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(20)?
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
	let map = file.lines().enumerate().flat_map(|(y, line)| {
		line.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_, _>>();

	let end = map.iter().find(|x| x.1 == &'E').unwrap().0.clone();

	let mut best_way: HashMap<(i32, i32), i32> = HashMap::new();

	let mut queue = VecDeque::new();
	queue.push_back((end, 0));

	while let Some((pos, time)) = queue.pop_front() {
		if best_way.contains_key(&pos) {
			continue;
		}
		best_way.insert(pos, time);

		[
			(pos.0 + 1, pos.1),
			(pos.0 - 1, pos.1),
			(pos.0, pos.1 + 1),
			(pos.0, pos.1 - 1),
		].into_iter().filter(|x| map.contains_key(x) && map[x] != '#').for_each(|xy| queue.push_back((xy, time + 1)));
	}

	let starts = map.iter().filter(|x| x.1 != &'#').map(|x|x.0.clone());

	let mut skips = Vec::new();

	for pos in starts {
		let best = best_way[&pos];
		[
			(pos.0 + 2, pos.1),
			(pos.0 - 2, pos.1),
			(pos.0, pos.1 + 2),
			(pos.0, pos.1 - 2),
			(pos.0 - 1, pos.1 + 1),
			(pos.0 - 1, pos.1 - 1),
			(pos.0 + 1, pos.1 - 1),
			(pos.0 + 1, pos.1 + 1),
		].into_iter()
			.filter(|x| best_way.contains_key(&x))
			.map(|x| best - best_way[&x] - 2)
			.filter(|x| *x > 0)
			.for_each(|x| skips.push(x));
	}

	skips.sort();
	
	let res = skips.iter().filter(|&&x| x >= 100).count();

	return res.to_string();
}

fn part2(file: String) -> String {
	let map = file.lines().enumerate().flat_map(|(y, line)| {
		line.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_, _>>();

	let end = map.iter().find(|x| x.1 == &'E').unwrap().0.clone();

	let mut best_way: HashMap<(i32, i32), i32> = HashMap::new();

	let mut queue = VecDeque::new();
	queue.push_back((end, 0));

	while let Some((pos, time)) = queue.pop_front() {
		if best_way.contains_key(&pos) {
			continue;
		}
		best_way.insert(pos, time);

		[
			(pos.0 + 1, pos.1),
			(pos.0 - 1, pos.1),
			(pos.0, pos.1 + 1),
			(pos.0, pos.1 - 1),
		].into_iter().filter(|x| map.contains_key(x) && map[x] != '#').for_each(|xy| queue.push_back((xy, time + 1)));
	}

	let nodes = map.iter().filter(|x| x.1 != &'#').map(|x|x.0.clone()).collect::<Vec<_>>();

	let mut skips = Vec::new();

	let max_length = 20;

	for &pos in &nodes {
		let best = best_way[&pos];

		nodes.iter()
			.map(|x| (x.clone(), pos.0.abs_diff(x.0) as i32 + pos.1.abs_diff(x.1) as i32))
			.filter(|&(_, length)| length <= max_length)
			.map(|(x, length)| best - best_way[&x] - length)
			.filter(|x| *x > 0)
			.for_each(|x| skips.push(x));
	}

	skips.sort();
	let _debug = skips.chunk_by(|x, b| x == b).map(|x| (x[0], x.iter().count())).collect::<Vec<_>>();
	
	let res = skips.iter().filter(|&&x| x >= 100).count();

	return res.to_string();
}