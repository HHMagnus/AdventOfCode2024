use std::cmp::Ordering;

use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(5)?
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
	let mut split = file.split("\n\n");
	let rules = split.next().unwrap().lines().map(|line| {
		let mut split = line.split("|");
		let x1 = split.next().unwrap().parse::<usize>().unwrap();
		let x2 = split.next().unwrap().parse::<usize>().unwrap();
		(x1, x2)
	}).collect::<Vec<_>>();
	let lines = split.next().unwrap().lines().map(|line| {
		line.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>()
	}).collect::<Vec<_>>();
	let res = lines.into_iter().map(|line| {
		for i in 0..line.len() {
			let curr = line[i];
			let rules = rules.iter().filter(|x| x.0 == curr).map(|x| x.1).collect::<Vec<_>>();
			for j in 0..i {
				let prev = line[j];
				if rules.contains(&prev) {
					return 0;
				}
			}
		}
		return line[line.len()/2];
	}).sum::<usize>();
	return res.to_string();
}

fn part2(file: String) -> String {
	let mut split = file.split("\n\n");
	let rules = split.next().unwrap().lines().map(|line| {
		let mut split = line.split("|");
		let x1 = split.next().unwrap().parse::<usize>().unwrap();
		let x2 = split.next().unwrap().parse::<usize>().unwrap();
		(x1, x2)
	}).collect::<Vec<_>>();
	let lines = split.next().unwrap().lines().map(|line| {
		line.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>()
	}).collect::<Vec<_>>();
	let res = lines.into_iter().filter(|line| {
		for i in 0..line.len() {
			let curr = line[i];
			let rules = rules.iter().filter(|x| x.0 == curr).map(|x| x.1).collect::<Vec<_>>();
			for j in 0..i {
				let prev = line[j];
				if rules.contains(&prev) {
					return true;
				}
			}
		}
		return false;
	}).map(|mut line| {
		line.sort_by(|&a, &b| if rules.iter().any(|x| x.0 == b && x.1 == a) { Ordering::Less } else { Ordering::Greater });
		return line[line.len()/2];
	}).sum::<usize>();
	return res.to_string();
}