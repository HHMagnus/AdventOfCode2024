use std::collections::{BTreeMap, BTreeSet, VecDeque};

use aoc_client::{AocClient, AocResult};
use itertools::Itertools;

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(23)?
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
	let input = file.lines().map(|x| {
		let mut split = x.split("-");
		(split.next().unwrap(), split.next().unwrap())
	}).collect::<Vec<_>>();

	let mut map = BTreeMap::new();
	
	for i in input {
		map.entry(i.0).or_insert(BTreeSet::new()).insert(i.1);
		map.entry(i.1).or_insert(BTreeSet::new()).insert(i.0);
	}

	let map = map.into_iter().map(|x| (x.0, x.1.into_iter().collect::<Vec<_>>())).collect::<BTreeMap<_,_>>();

	let mut sets = BTreeSet::new();
	for &x in map.keys() {
		if !x.starts_with("t") {
			continue;
		}

		let set = &map[x];

		for i in 0..set.len() {
			for j in i+1..set.len() {
				let y = set[i];
				let y_set = &map[y];
				let z = set[j];
				if y_set.contains(&z) {
					let mut vec = vec![x, y, z];
					vec.sort();
					sets.insert(vec);
				}
			}
		}
	}
	let res = sets.len();
	return res.to_string();
}

fn part2(file: String) -> String {
	let input = file.lines().map(|x| {
		let mut split = x.split("-");
		(split.next().unwrap(), split.next().unwrap())
	}).collect::<Vec<_>>();

	let mut map = BTreeMap::new();
	
	for i in input {
		map.entry(i.0).or_insert(BTreeSet::new()).insert(i.1);
		map.entry(i.1).or_insert(BTreeSet::new()).insert(i.0);
	}

	let map = map.into_iter().map(|x| (x.0, x.1.into_iter().collect::<Vec<_>>())).collect::<BTreeMap<_,_>>();

	let sets = map.keys().flat_map(|&x| sets(&map, x)).collect::<BTreeSet<_>>();
	let res = sets.into_iter().max_by_key(|x| x.len()).unwrap().into_iter().sorted().collect::<Vec<_>>().join(",");
	return res.to_string();
}

fn sets<'a>(map: &BTreeMap<&'a str, Vec<&'a str>>, curr: &'a str) -> BTreeSet<Vec<&'a str>> {
	let mut result = BTreeSet::new();

	let mut queue: VecDeque<(&'a str, Vec<&'a str>)> = VecDeque::new();
	queue.push_back((curr, vec![curr]));

	'b: while let Some((curr, prev)) = queue.pop_front() {
		for &y in &map[curr] {
			if prev.contains(&y) {
				continue 'b;
			}

			if prev.iter().all(|z| map[z].contains(&y)) {
				let mut cloned = prev.clone();
				cloned.push(&y);
				result.insert(cloned.clone());
				queue.push_back((y, cloned));
			}
		}
	}
	
	result
}