use std::collections::HashMap;

use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(22)?
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
	let nums = file.lines().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
	let prune = 16777216;
	let res = nums.into_iter().map(|num| {
		let mut secret_number = num;
		for _ in 0..2000 {
			secret_number = secret_number ^ (secret_number * 64);
			secret_number %= prune;
			secret_number = secret_number ^ (secret_number / 32);
			secret_number %= prune;
			secret_number = secret_number ^ (secret_number * 2048);
			secret_number %= prune;
		}
		secret_number
	}).sum::<i64>();
	return res.to_string();
}

fn part2(file: String) -> String {
	let nums = file.lines().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
	let prune = 16777216;
	let nums = nums.into_iter().map(|num| {
		let mut nums = Vec::new();
		let mut secret_number = num;
		nums.push(secret_number);
		for _ in 0..2000 {
			secret_number = secret_number ^ (secret_number * 64);
			secret_number %= prune;
			secret_number = secret_number ^ (secret_number / 32);
			secret_number %= prune;
			secret_number = secret_number ^ (secret_number * 2048);
			secret_number %= prune;
			nums.push(secret_number);
		}
		nums
	}).collect::<Vec<_>>();
	let mut map = HashMap::new();

	for nums in nums.into_iter() {
		let mut local = HashMap::new();
		let nums = nums.into_iter().map(|x| x % 10).collect::<Vec<_>>();
		for x in nums.windows(5) {
			let vec = vec![
				x[1] - x[0],
				x[2] - x[1],
				x[3] - x[2],
				x[4] - x[3]
			];
			let num = x[4];
			if local.contains_key(&vec) {
				continue;
			}
			local.insert(vec, num);
		}

		for l in local {
			*map.entry(l.0).or_insert(0) += l.1;
		}
	}
	
	let res = map.into_iter().map(|x| x.1).max().unwrap();
	return res.to_string();
}