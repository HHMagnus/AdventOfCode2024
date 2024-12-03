
use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(3)?
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
	return muls(file).to_string();
}

fn muls(file: String) -> usize {
	let mut res = 0;
	let mut split = file.split("mul(");
	while let Some(x) = split.next() {
		let mut s = x.split(",");
		let x1 = s.next().and_then(|l| l.parse::<usize>().map_or(None, |x| Some(x)));
		let x2 = s.next().and_then(|l| {
			for x in l.chars() {
				if x.is_digit(10) {
					continue;
				}

				if x == ')' {
					return l.split(")").next()
				}
				return None;
			}
			return None;
			
		}).and_then(|l| l.parse::<usize>().map_or(None, |x| Some(x)));
		if let Some(val) = x1.and_then(|x| x2.map(|y| x * y)) {
			res += val;
		}
	}
	res
}

fn part2(file: String) -> String {
	let mut res = 0;
	let mut do_split = file.split("do()");
	while let Some(do_) = do_split.next() {
		res += do_.split("don't()").next().map_or(0, |x| muls(x.to_string()));
	}
	return res.to_string();
}