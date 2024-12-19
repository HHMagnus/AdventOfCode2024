use std::{cmp::Reverse, collections::{BinaryHeap, VecDeque}};

use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(19)?
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

	let lines = file.lines().collect::<Vec<_>>();
	let input = lines[0].split(", ").collect::<Vec<_>>();

	let mut res = 0;
	'o: for &line in lines[2..].iter() {
		println!("{}", line);
		let mut curr = VecDeque::new();
		curr.push_back(0);

		while let Some(pos) = curr.pop_front() {
			let sub = &line[pos..];
			for &i in &input {
				if sub.starts_with(i) {
					curr.push_front(pos + i.len());
				}
				if sub == i {
					res += 1;
					continue 'o;
				}
			}
		}
	}
	return res.to_string();
}

fn part2(file: String) -> String {
	let lines = file.lines().collect::<Vec<_>>();
	let input = lines[0].split(", ").collect::<Vec<_>>();

	let mut res: usize = 0;
	for &line in lines[2..].iter() {
		println!("{}", line);
		let mut curr = BinaryHeap::new();
		curr.push((Reverse(0), 1));

		while let Some((Reverse(pos), mut len)) = curr.pop() {
			loop {
				let peek = curr.peek();
				if peek.is_some() && peek.unwrap().0 == Reverse(pos) {
					len += peek.unwrap().1;
					curr.pop();
				} else {
					break;
				}
			}
			let sub = &line[pos..];
			for &i in &input {
				if sub.starts_with(i) {
					curr.push((Reverse(pos + i.len()), len));
				}
				if sub == i {
					res += len;
				}
			}
		}
	}
	return res.to_string();
}