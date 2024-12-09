use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(9)?
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
	let arr = file.chars().filter_map(|x| x.to_digit(10)).map(|x| x as usize).collect::<Vec<_>>();
	let mut block_id = 0;
	let mut constructed = Vec::new();
	for i in 0..arr.len() {
		let ins = if i % 2 == 0 {
			block_id += 1;
			Some(block_id-1)
		} else {
			None
		};

		let num = arr[i];
		for _ in 0..num {
			constructed.push(ins);
		}

	}

	let mut front = 0;
	let mut back = constructed.len()-1;

	while front < back {
		while constructed[front] != None {
			front += 1;
		}

		while constructed[back] == None {
			back -= 1;
		}
		if front >= back { break; }
		constructed[front] = constructed[back];
		constructed[back] = None;

		front += 1;
		back -= 1;
	}

	let mut res = 0;
	for (i, x) in constructed.into_iter().enumerate() {
		if let Some(x) = x {
			res += i * x;
		}
	}
	return res.to_string();
}

fn part2(file: String) -> String {
	let arr = file.chars().filter_map(|x| x.to_digit(10)).map(|x| x as usize).collect::<Vec<_>>();
	let mut block_id = 0;
	let mut constructed = Vec::new();
	for i in 0..arr.len() {
		let ins = if i % 2 == 0 {
			block_id += 1;
			Some(block_id-1)
		} else {
			None
		};

		let num = arr[i];
		for _ in 0..num {
			constructed.push(ins);
		}
	}

	let mut back = constructed.len()-1;
	while back > 0 {
		while constructed[back] == None {
			back -= 1;
		}

		let elem = constructed[back];
		let mut block = 1;
		let mut i = back-1;
		while i > 0 && constructed[i] == elem {
			block += 1;
			i -= 1;
		}
		
		let mut front = 0;
		while front < back {
			while constructed[front] != None {
				front += 1;
			}

			if front >= constructed.len() { break; }

			let mut frees = 1;
			let mut i = front+1;
			while i < constructed.len() && constructed[i] == None {
				frees += 1;
				i += 1;
			}

			if front >= back {
				break;
			}

			if frees >= block {
				for x in 0..block {
					constructed[front + x] = constructed[back - x];
					constructed[back - x] = None;
				}
				break;
			} else {
				front += frees;
			}
		}
		back -= block;
	}

	let mut res = 0;
	for (i, x) in constructed.into_iter().enumerate() {
		if let Some(x) = x {
			res += i * x;
		}
	}
	return res.to_string();
}