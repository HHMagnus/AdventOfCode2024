
use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(1)?
        .build()?;

    let input: String = client.get_input()?;

    let answer_part1 = part1(input.clone());
    client.submit_answer(1, answer_part1)?;
	
    let answer_part2 = part2(input);
    client.submit_answer(2, answer_part2)?;

	Ok(())
}
fn part1(file: String) -> String {
	let input = file.lines().map(|line| {
		let mut split = line.split("   ");
		(split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap())
	}).collect::<Vec<_>>();
	let mut list1 = input.iter().map(|x| x.0).collect::<Vec<_>>();
	let mut list2 = input.iter().map(|x| x.1).collect::<Vec<_>>();
	list1.sort();
	list2.sort();

	let res = list1.iter().zip(list2).map(|x| x.0.abs_diff(x.1)).sum::<u32>();
	return res.to_string();
}

fn part2(file: String) -> String {
	let input = file.lines().map(|line| {
		let mut split = line.split("   ");
		(split.next().unwrap().parse::<i32>().unwrap(), split.next().unwrap().parse::<i32>().unwrap())
	}).collect::<Vec<_>>();
	let list1 = input.iter().map(|x| x.0).collect::<Vec<_>>();
	let list2 = input.iter().map(|x| x.1).collect::<Vec<_>>();

	let res = list1.into_iter().map(|x| list2.iter().filter(|&&y| y == x).count() as i32 * x).sum::<i32>();
	return res.to_string();
}