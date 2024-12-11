use aoc_client::{AocClient, AocResult};
use itertools::Itertools;

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(11)?
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
	let mut stone = file.lines().next().unwrap().split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();

	for _ in 0..25 {
		stone = stone.into_iter().flat_map(|stone| {
			if stone == 0 {
				return vec![1];
			}

			let str = stone.to_string();
			if str.len() % 2 == 0 {
				return vec![str[0..str.len()/2].parse().unwrap(), str[str.len()/2..str.len()].parse().unwrap()];
			}
			return vec![stone * 2024];
		}).collect();
	}
	let res = stone.len();
	return res.to_string();
}

fn part2(file: String) -> String {
	let stone = file.lines().next().unwrap().split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
	
	let mut stone = stone.into_iter().chunk_by(|&x| x).into_iter().map(|x| (x.0, x.1.count())).collect::<Vec<_>>();

	for _ in 0..75 {
		stone = stone.into_iter().flat_map(|stone| {
			if stone.0 == 0 {
				return vec![(1, stone.1)];
			}

			let str = stone.0.to_string();
			if str.len() % 2 == 0 {
				return vec![(str[0..str.len()/2].parse().unwrap(), stone.1), (str[str.len()/2..str.len()].parse().unwrap(), stone.1)];
			}
			return vec![(stone.0 * 2024, stone.1)];
		}).sorted().chunk_by(|&x| x.0).into_iter().map(|x| (x.0, x.1.map(|x| x.1).sum())).collect();
	}
	let res = stone.into_iter().map(|x| x.1).sum::<usize>();
	return res.to_string();
}
