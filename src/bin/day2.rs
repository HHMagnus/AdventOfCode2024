use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(2)?
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
	let res = file.lines().filter(|line| {
		let list = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
		let mut other1 = list.clone();
		other1.sort();
		let mut other2 = list.clone();
		other2.sort();
		other2.reverse();
		(list == other1 || list == other2)
		&& list.windows(2).all(|x| x[0].abs_diff(x[1]) >= 1 && x[0].abs_diff(x[1]) <= 3 )
	}).count();
	return res.to_string();
}

fn part2(file: String) -> String {
	let res = file.lines().filter(|line| {
		let list = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
		for i in 0..list.len() {
			let list = list[0..i].iter().chain(list[i+1..list.len()].iter()).cloned().collect::<Vec<_>>();
			let mut other1 = list.clone();
			other1.sort();
			let mut other2 = list.clone();
			other2.sort();
			other2.reverse();
			if (list == other1 || list == other2)
				&& list.windows(2).all(|x| x[0].abs_diff(x[1]) >= 1 && x[0].abs_diff(x[1]) <= 3 ) {
					return true;
			}
		}
		false
	}).count();
	return res.to_string();
}