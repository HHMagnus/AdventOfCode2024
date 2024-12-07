use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(7)?
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
	let lines = file.lines().map(|line| {
		let mut split = line.split(": ");
		let first = split.next().unwrap().parse::<i64>().unwrap();
		let list = split.next().unwrap().split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
		(first, list)
	}).collect::<Vec<_>>();

	let res = lines.into_iter().map(|(sum, list)| {
		let mut iter = list.into_iter();
		let mut xs = vec![iter.next().unwrap()];
		while let Some(next) = iter.next() {
			xs = xs.into_iter().flat_map(|x| {
				vec![x * next, x + next]
			}).collect::<Vec<_>>();
		}
		if xs.contains(&sum) {
			return sum
		} else {
			return 0
		}
	}).sum::<i64>();
	return res.to_string();
}

fn part2(file: String) -> String {
	let lines = file.lines().map(|line| {
		let mut split = line.split(": ");
		let first = split.next().unwrap().parse::<i64>().unwrap();
		let list = split.next().unwrap().split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
		(first, list)
	}).collect::<Vec<_>>();

	let res = lines.into_iter().map(|(sum, list)| {
		let mut iter = list.into_iter();
		let mut xs = vec![iter.next().unwrap()];
		while let Some(next) = iter.next() {
			xs = xs.into_iter().flat_map(|x| {
				let comb = x.to_string() + &next.to_string();
				vec![x * next, x + next, comb.parse::<i64>().unwrap()]
			}).collect::<Vec<_>>();
		}
		if xs.contains(&sum) {
			return sum
		} else {
			return 0
		}
	}).sum::<i64>();
	return res.to_string();
}