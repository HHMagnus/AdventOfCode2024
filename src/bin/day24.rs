use std::collections::{HashMap, HashSet};

use aoc_client::{AocClient, AocResult};
use itertools::Itertools;

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(24)?
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rule {
	And,
	Or,
	Xor,
}

fn part1(file: String) -> String {
	let mut split = file.split("\n\n");
	let map = split.next().unwrap().lines().map(|x| {
		let mut split = x.split(": ");
		(split.next().unwrap(), split.next().unwrap() == "1")
	}).collect::<HashMap<_,_>>();
	let rules = split.next().unwrap().lines().map(|x| {
		let mut split = x.split(" -> ");
		let s2 = split.next().unwrap();
		let res = split.next().unwrap();
		let (mut split, rule) = if s2.contains("AND") {
			(s2.split(" AND "), Rule::And)
		} else if s2.contains("XOR") {
			(s2.split(" XOR "), Rule::Xor)
		} else if s2.contains("OR") {
			(s2.split(" OR "), Rule::Or)
		} else {
			panic!("Unknown {}", s2);
		};
		(split.next().unwrap(), split.next().unwrap(), rule, res)
	}).collect::<Vec<_>>();
	let res = num(&calculate(map, rules));
	return res.to_string();
}

fn calculate<'a>(mut map: HashMap<&'a str, bool>, rules: Vec<(&'a str, &'a str, Rule, &'a str)>) -> Vec<bool> {
	let zs = rules.iter().filter(|x|x.3.starts_with("z")).map(|x|x.3).sorted().rev().collect::<Vec<_>>();

	while !zs.iter().all(|z| map.contains_key(z)) {
			for &rule in &rules {
				if !map.contains_key(rule.0) || !map.contains_key(rule.1) || map.contains_key(rule.3) {
					continue;
				}

				let i1 = map[rule.0];
				let i2 = map[rule.1];
				let res = match rule.2 {
					Rule::And => i1 && i2,
					Rule::Or => i1 || i2,
					Rule::Xor => i1 != i2,
				};
				map.insert(rule.3, res);
			}
		}

	parse(&map, zs)
}
fn num(vec: &Vec<bool>) -> isize {
	isize::from_str_radix(&vec.into_iter().map(|x| if *x { '1' } else { '0' }).collect::<String>(), 2).unwrap()
}

fn parse<'a>(map: &HashMap<&'a str, bool>, xs: Vec<&'a str>) -> Vec<bool> {
	xs.into_iter().map(|x| map[x]).collect::<Vec<_>>()
}

fn part2(file: String) -> String {
	let mut split = file.split("\n\n");
	let map = split.next().unwrap().lines().map(|x| {
		let mut split = x.split(": ");
		(split.next().unwrap(), split.next().unwrap() == "1")
	}).collect::<HashMap<_,_>>();
	let rules = split.next().unwrap().lines().map(|x| {
		let mut split = x.split(" -> ");
		let s2 = split.next().unwrap();
		let res = split.next().unwrap();
		let (mut split, rule) = if s2.contains("AND") {
			(s2.split(" AND "), Rule::And)
		} else if s2.contains("XOR") {
			(s2.split(" XOR "), Rule::Xor)
		} else if s2.contains("OR") {
			(s2.split(" OR "), Rule::Or)
		} else {
			panic!("Unknown {}", s2);
		};
		(split.next().unwrap(), split.next().unwrap(), rule, res)
	}).collect::<Vec<_>>();

	let mut incorrect = Vec::new();
	for &rule in &rules {
		// Xor is used to store the binary result for all but the last one. (The last one should only hold the carry over)
		if rule.3.starts_with("z") && rule.2 != Rule::Xor && rule.3 != "z45" {
			incorrect.push(rule.3);
		}
		// Continuation of previous
		if rule.2 == Rule::Xor && !rule.0.starts_with("x") && !rule.0.starts_with("y") && !rule.1.starts_with("x") && !rule.1.starts_with("y") && !rule.3.starts_with("z") {
			incorrect.push(rule.3);
		}
		// And should be used to store the carry over so should never be used after or
		if rule.2 == Rule::And && rule.0 != "x00" && rule.1 != "x00" {
			for &rule2 in &rules {
				if (rule.3 == rule2.0 || rule.3 == rule2.1) && rule2.2 != Rule::Or {
					incorrect.push(rule.3);
				}
			}
		}
		// Or is used to handle carry over combination in cases of three 1s, so should come before the XOR.
		if rule.2 == Rule::Xor {
			for &rule2 in &rules {
				if (rule.3 == rule2.0 || rule.3 == rule2.1) && rule2.2 == Rule::Or {
					incorrect.push(rule.3);
				}
			}
		}
	}

	incorrect = incorrect.into_iter().collect::<HashSet<_>>().into_iter().sorted().collect::<Vec<_>>();
	let res = incorrect.join(",");
	return res.to_string();
}