use aoc_client::{AocClient, AocResult};
use itertools::Itertools;

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(17)?
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
	let mut a = lines[0].replace("Register A: ", "").parse::<usize>().unwrap();
	let mut b = lines[1].replace("Register B: ", "").parse::<usize>().unwrap();
	let mut c = lines[2].replace("Register C: ", "").parse::<usize>().unwrap();
	let program = lines[4].replace("Program: ", "").split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

	let mut output = Vec::new();

	let mut i = 0;

	while i < program.len()-1 {
		let opcode = program[i];
		let literal = program[i+1];

		let combo = if literal == 4 { a } else if literal == 5 { b } else if literal == 6 { c } else { literal };

		let base: usize = 2;
		if opcode == 0 {
			a = a / (base.pow(combo as u32));
		} else if opcode == 1 {
			b = b ^ literal;
		} else if opcode == 2 {
			b = combo % 8;
		} else if opcode == 3 {
			if a != 0 {
				i = literal;
				continue;
			}
		} else if opcode == 4 {
			b = b ^ c;
		} else if opcode == 5 {
			output.push(combo % 8);
		} else if opcode == 6 {
			b = a / (base.pow(combo as u32));
		} else if opcode == 7 {
			c = a / (base.pow(combo as u32));
		}
		i += 2;
	}
	
	return output.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
}

fn part2(file: String) -> String {let lines = file.lines().collect::<Vec<_>>();
	let program = lines[4].replace("Program: ", "").split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

	let mut ress = vec![0];

	for inst in program.into_iter().rev() {
		let mut new = Vec::new();
		for res in &ress {
			let xs = (0..8).filter_map(|trying| {
				let a = (res << 3) | trying;
				if a == 0 { return None; }
				let b = trying;
				let b = b ^ 2;
				let c = a >> b;
				let b = b ^ c;
				let b = b ^ 3;
				if b % 8 == inst { return Some(a) }
				None
			}).collect::<Vec<_>>();
			new.push(xs);
		}

		ress = new.into_iter().flatten().collect();
	}

	// Input program
	// b = a % 8
	// b = b ^ 2
	// c = a / 2.pow(b)
	// b = b ^ c
	// b = b ^ 3
	// out b % 8
	// a = a / 8
	// restart

	return ress[0].to_string();
}