use std::u64;

use aoc_client::{AocClient, AocResult};
use itertools::Itertools;

fn main() -> AocResult<()> {
    let client = AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(2024)?
        .day(14)?
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
	let mut robots = file.lines().map(|line| {
		let mut split = line.split(" v=");
		let mut first = split.next().unwrap().split(",");
		let px = first.next().unwrap().replace("p=", "").parse::<i64>().unwrap();
		let py = first.next().unwrap().parse::<i64>().unwrap();

		let mut second = split.next().unwrap().split(",");
		let vx = second.next().unwrap().parse::<i64>().unwrap();
		let vy = second.next().unwrap().parse::<i64>().unwrap();

		((px, py), (vx, vy))
	}).collect::<Vec<_>>();

	let maxx = 101;
	let maxy = 103;

	for _ in 0..100 {
		robots = robots.into_iter().map(|(pos, vel)| {
			let x = pos.0 + vel.0;
			let mut x = x % maxx;
			while x < 0 {
				x += maxx;
			}
			let y = pos.1 + vel.1;
			let mut y = y % maxy;
			while y < 0 {
				y += maxy;
			}
			let newpos = (x, y);
			(newpos, vel)
		}).collect();
	}

	let mut q1 = 0;
	let mut q2 = 0;
	let mut q3 = 0;
	let mut q4 = 0;
	for robot in robots {
		if robot.0.0 < maxx/2 {
			if robot.0.1 < maxy/2 {
				q1+=1;
			} else if robot.0.1 > maxy/2 {
				q2+=1;
			}
		} else if robot.0.0 > maxx/2 {
			if robot.0.1 < maxy/2 {
				q3+=1;
			} else if robot.0.1 > maxy/2 {
				q4+=1;
			}
		}
	}


	let res = q1 * q2 * q3 * q4;
	return res.to_string();
}

fn part2(file: String) -> String {
	let mut robots = file.lines().map(|line| {
		let mut split = line.split(" v=");
		let mut first = split.next().unwrap().split(",");
		let px = first.next().unwrap().replace("p=", "").parse::<i64>().unwrap();
		let py = first.next().unwrap().parse::<i64>().unwrap();

		let mut second = split.next().unwrap().split(",");
		let vx = second.next().unwrap().parse::<i64>().unwrap();
		let vy = second.next().unwrap().parse::<i64>().unwrap();

		((px, py), (vx, vy))
	}).collect::<Vec<_>>();

	let maxx = 101;
	let maxy = 103;
	
	let mut time = 0;
	loop {
		robots.sort();
		let rough_distance = robots.windows(2).map(|x| x[0].0.0.abs_diff(x[1].0.0) + x[0].0.1.abs_diff(x[1].0.1)).sum::<u64>();
		if rough_distance < 7500 {
			break;
		}
		robots = robots.into_iter().map(|(pos, vel)| {
			let x = pos.0 + vel.0;
			let mut x = x % maxx;
			while x < 0 {
				x += maxx;
			}
			let y = pos.1 + vel.1;
			let mut y = y % maxy;
			while y < 0 {
				y += maxy;
			}
			let newpos = (x, y);
			(newpos, vel)
		}).collect();
		time += 1;
	}

	
	println!("{}", time);
	for y in 0..maxy {
		for x in 0..maxx {
			let row = robots.iter().filter(|d| d.0 == (x, y)).count();
			if row > 0 {
				print!("{}", row);
			} else {
				print!(".");
			}
		}
		println!("");
	}

	return time.to_string();
}