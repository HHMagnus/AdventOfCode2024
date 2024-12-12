use std::collections::{HashMap, HashSet, VecDeque};

use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(12)?
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
	let mut res = 0;
	let map = file.lines().enumerate().flat_map(|(y, line)| {
		line.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_,_>>();

	let mut visited_total = HashSet::new();

	for &pos in map.keys() {
		if visited_total.contains(&pos) {
			continue;
		}

		let typ = map[&pos];

		let mut queue = VecDeque::new();
		queue.push_back(pos);

		let mut other_typ = 0;
		let mut total = 0;

		while let Some(xy) = queue.pop_front() {
			if !map.contains_key(&xy) || map[&xy] != typ {
				other_typ += 1;
				continue;
			}

			if visited_total.contains(&xy) {
				continue;
			}
			visited_total.insert(xy);

			total += 1;

			[
				(xy.0 + 1, xy.1),
				(xy.0 - 1, xy.1),
				(xy.0, xy.1 + 1),
				(xy.0, xy.1 - 1)
			].into_iter().for_each(|x| queue.push_back(x));
		}

		res += other_typ * total;
	}

	return res.to_string();
}

fn part2(file: String) -> String {
	let mut res = 0;
	let map = file.lines().enumerate().flat_map(|(y, line)| {
		line.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c))
	}).collect::<HashMap<_,_>>();

	let mut visited_total = HashSet::new();

	for &pos in map.keys() {
		if visited_total.contains(&pos) {
			continue;
		}

		let typ = map[&pos];

		let mut queue = VecDeque::new();
		queue.push_back(pos);

		let mut other_typ = 0;
		let mut total = 0;

		let mut other_typ_pos = Vec::new();

		while let Some(xy) = queue.pop_front() {
			if !map.contains_key(&xy) || map[&xy] != typ {
				other_typ += 1;
				continue;
			}

			if visited_total.contains(&xy) {
				continue;
			}
			visited_total.insert(xy);

			total += 1;

			[
				(1, 0),
				(-1, 0),
				(0, 1),
				(0, -1)
			].into_iter().for_each(|x| {
				let pos = (x.0 + xy.0, x.1 + xy.1);
				if !map.contains_key(&pos) || map[&pos] != typ {
					other_typ_pos.push((x, pos));
				}
				queue.push_back(pos)
			});
		}

		for i in 0..other_typ_pos.len() {
			for j in i+1..other_typ_pos.len() {
				let e1 = other_typ_pos[i];
				let e2 = other_typ_pos[j];

				let is_neighbours = [
					(e1.1.0 + 1, e1.1.1),
					(e1.1.0 - 1, e1.1.1),
					(e1.1.0, e1.1.1 + 1),
					(e1.1.0, e1.1.1 - 1),
				].contains(&e2.1);

				if is_neighbours && e1.0 == e2.0 {
					other_typ -= 1;
				}
			}
		}

		res += other_typ * total;
	}

	return res.to_string();
}