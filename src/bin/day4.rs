use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(4)?
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

fn xmas(chars: &Vec<char>) -> usize {
	chars.windows(4).filter(|x| x[0] == 'X' && x[1] == 'M' && x[2] == 'A' && x[3] == 'S').count()
}

fn part1(file: String) -> String {
	let mut matrix = file.lines().map(|line| {
		line.chars().collect::<Vec<_>>()
	}).collect::<Vec<_>>();

	let mut res = 0;
	for line in matrix.iter() {
		res += xmas(line);
		let mut rev = line.clone();
		rev.reverse();
		res += xmas(&rev);
	}

	for x in 0..matrix[0].len() {
		let mut list = (0..matrix.len()).map(|y| matrix[y][x]).collect::<Vec<_>>();
		res += xmas(&list);
		list.reverse();
		res += xmas(&list);
	}

	for y in 0..matrix.len() {
		for x in 0..matrix[0].len() {
			if matrix.get(y).and_then(|cs| cs.get(x)) == Some(&'X')
				&& matrix.get(y+1).and_then(|cs| cs.get(x+1)) == Some(&'M')
				&& matrix.get(y+2).and_then(|cs| cs.get(x+2)) == Some(&'A')
				&& matrix.get(y+3).and_then(|cs| cs.get(x+3)) == Some(&'S') {
					res += 1;
				}
		}
	}

	for y in 0..matrix.len() {
		for x in 0..matrix[0].len() {
			if matrix.get(y).and_then(|cs| cs.get(x)) == Some(&'S')
				&& matrix.get(y+1).and_then(|cs| cs.get(x+1)) == Some(&'A')
				&& matrix.get(y+2).and_then(|cs| cs.get(x+2)) == Some(&'M')
				&& matrix.get(y+3).and_then(|cs| cs.get(x+3)) == Some(&'X') {
					res += 1;
				}
		}
	}
	
	for i in 0..matrix.len() {
		matrix[i].reverse();
	}

	for y in 0..matrix.len() {
		for x in 0..matrix[0].len() {
			if matrix.get(y).and_then(|cs| cs.get(x)) == Some(&'X')
				&& matrix.get(y+1).and_then(|cs| cs.get(x+1)) == Some(&'M')
				&& matrix.get(y+2).and_then(|cs| cs.get(x+2)) == Some(&'A')
				&& matrix.get(y+3).and_then(|cs| cs.get(x+3)) == Some(&'S') {
					res += 1;
				}
		}
	}

	for y in 0..matrix.len() {
		for x in 0..matrix[0].len() {
			if matrix.get(y).and_then(|cs| cs.get(x)) == Some(&'S')
				&& matrix.get(y+1).and_then(|cs| cs.get(x+1)) == Some(&'A')
				&& matrix.get(y+2).and_then(|cs| cs.get(x+2)) == Some(&'M')
				&& matrix.get(y+3).and_then(|cs| cs.get(x+3)) == Some(&'X') {
					res += 1;
				}
		}
	}
	
	return res.to_string();
}

fn part2(file: String) -> String {
	let matrix = file.lines().map(|line| {
		line.chars().collect::<Vec<_>>()
	}).collect::<Vec<_>>();

	let mut res = 0;

	for y in 0..matrix.len() {
		for x in 0..matrix[0].len() {
			if matrix.get(y).and_then(|cs| cs.get(x)) == Some(&'M')
				&& matrix.get(y+1).and_then(|cs| cs.get(x+1)) == Some(&'A')
				&& matrix.get(y+2).and_then(|cs| cs.get(x)) == Some(&'M')
				&& matrix.get(y).and_then(|cs| cs.get(x+2)) == Some(&'S')
				&& matrix.get(y+2).and_then(|cs| cs.get(x+2)) == Some(&'S') {
					res += 1;
				}
		}
	}

	for y in 0..matrix.len() {
		for x in 0..matrix[0].len() {
			if matrix.get(y).and_then(|cs| cs.get(x)) == Some(&'S')
				&& matrix.get(y+1).and_then(|cs| cs.get(x+1)) == Some(&'A')
				&& matrix.get(y+2).and_then(|cs| cs.get(x)) == Some(&'S')
				&& matrix.get(y).and_then(|cs| cs.get(x+2)) == Some(&'M')
				&& matrix.get(y+2).and_then(|cs| cs.get(x+2)) == Some(&'M') {
					res += 1;
				}
		}
	}

	for y in 0..matrix.len() {
		for x in 0..matrix[0].len() {
			if matrix.get(y).and_then(|cs| cs.get(x)) == Some(&'M')
				&& matrix.get(y+1).and_then(|cs| cs.get(x+1)) == Some(&'A')
				&& matrix.get(y+2).and_then(|cs| cs.get(x)) == Some(&'S')
				&& matrix.get(y).and_then(|cs| cs.get(x+2)) == Some(&'M')
				&& matrix.get(y+2).and_then(|cs| cs.get(x+2)) == Some(&'S') {
					res += 1;
				}
		}
	}

	for y in 0..matrix.len() {
		for x in 0..matrix[0].len() {
			if matrix.get(y).and_then(|cs| cs.get(x)) == Some(&'S')
				&& matrix.get(y+1).and_then(|cs| cs.get(x+1)) == Some(&'A')
				&& matrix.get(y+2).and_then(|cs| cs.get(x)) == Some(&'M')
				&& matrix.get(y).and_then(|cs| cs.get(x+2)) == Some(&'S')
				&& matrix.get(y+2).and_then(|cs| cs.get(x+2)) == Some(&'M') {
					res += 1;
				}
		}
	}

	return res.to_string();
}