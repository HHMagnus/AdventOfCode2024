use aoc_client::{AocClient, AocResult};

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(13)?
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
	let mut res = 0;

	let lines = parse(file);

	for line in lines {
		'c: for x in 0..100 {
			for y in 0..100 {
				let c1 = x * line.0.0 + y * line.1.0 == line.2.0;
				let c2 = x * line.0.1 + y * line.1.1 == line.2.1;
				if c1 && c2 {
					res += x*3 + y;
					break 'c;
				}
			}
		}
	}

	return res.to_string();
}

fn parse(file: String) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    let lines = file.split("\n\n").into_iter().map(|part| {
		    let lines = part.lines().collect::<Vec<_>>();
		    let button_a = lines[0];
		    let mut a = button_a.split(", Y+");
		    let a1 = a.next().unwrap().replace("Button A: X+", "").parse::<i64>().unwrap();
		    let a2 = a.next().unwrap().parse::<i64>().unwrap();
		    let button_b = lines[1];
		    let mut b = button_b.split(", Y+");
		    let b1 = b.next().unwrap().replace("Button B: X+", "").parse::<i64>().unwrap();
		    let b2 = b.next().unwrap().parse::<i64>().unwrap();
		    let prize = lines[2];
		    let mut p = prize.split(", Y=");
		    let p1 = p.next().unwrap().replace("Prize: X=", "").parse::<i64>().unwrap();
		    let p2 = p.next().unwrap().parse::<i64>().unwrap();

		    ((a1, a2), (b1, b2), (p1, p2))
	    }).collect::<Vec<_>>();
    lines
}

fn part2(file: String) -> String {
	let mut res = 0;

	let mul = 10000000000000;
	let lines: Vec<_> = parse(file).into_iter().map(|x| (x.0, x.1, (x.2.0 + mul, x.2.1 + mul))).collect();

	for line in lines {
		let ((ax, ay), (bx, by), (px, py)) = line;

		let y = (py * ax - px * ay) / (by * ax - bx * ay);
		let x = (px - y * bx) / ax;
		if (ax * x + bx * y, ay * x + by * y) != (px, py) {
			continue;
		}
		res += x * 3 + y;
	}

	return res.to_string();
}