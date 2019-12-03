fn main() {
	let input = std::fs::read_to_string("day_3/input.txt")
		.unwrap();

	let input = input.lines()
		.collect::<Vec<_>>();

	let mut v: Vec<((i64, i64), i64)> = Vec::new();
	let mut crosses: Vec<((i64, i64), i64)> = Vec::new();

	for (idx, line) in input.into_iter().enumerate() {
		let mut steps = 0;

		let mut p = (0i64, 0i64);

		for step in line.split(",") {
			let amount = step[1..].parse::<i64>().unwrap();
			match &step[0..1] {
				"R" => {
					for _ in 0..amount {
						steps += 1;
						p.0 += 1;
						if idx == 1 {
							if let Some((_, steps1)) = v.iter().find(|el| el.0 == p) {
								crosses.push((p, steps + *steps1));
							}
						} else {
							v.push((p, steps));
						}
					}
				}
				"L" => {
					for _ in 0..amount {
						steps += 1;
						p.0 -= 1;
						if idx == 1 {
							if let Some((_, steps1)) = v.iter().find(|el| el.0 == p) {
								crosses.push((p, steps + *steps1));
							}
						} else {
							v.push((p, steps));
						}
					}
				}
				"U" => {
					for _ in 0..amount {
						steps += 1;
						p.1 += 1;
						if idx == 1 {
							if let Some((_, steps1)) = v.iter().find(|el| el.0 == p) {
								crosses.push((p, steps + *steps1));
							}
						} else {
							v.push((p, steps));
						}
					}
				}
				"D" => {
					for _ in 0..amount {
						steps += 1;
						p.1 -= 1;
						if idx == 1 {
							if let Some((_, steps1)) = v.iter().find(|el| el.0 == p) {
								crosses.push((p, steps + *steps1));
							}
						} else {
							v.push((p, steps));
						}
					}
				}
				_ => (),
			}
		}
	}

	let p1 = crosses.iter().map(|p| manhattan_distance((0i64, 0i64), p.0)).min().unwrap();
	let p2 = crosses.into_iter().map(|p| p.1).min().unwrap();
	println!("Part 1: {:?}", p1);
	println!("Part 2: {:?}", p2);
}

fn manhattan_distance(p: (i64, i64), q: (i64, i64)) -> i64 {
	(p.0 - q.0).abs() + (p.1 + q.1).abs()
}
