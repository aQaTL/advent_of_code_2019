use itertools::Itertools;

fn main() {
	let re = regex::Regex::new(r"<x=([0-9-]+), y=([0-9-]+), z=([0-9-]+)>").unwrap();
	let input = std::fs::read_to_string("day_12/input.txt")
		.unwrap()
		.lines()
		.map(|line| {
			re.captures(line)
				.unwrap()
				.iter()
				.skip(1)
				.filter_map(|e| e.map(|e| e.as_str().parse::<i64>().ok()).unwrap())
				.chain((0..1).cycle().take(3))
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	println!("Part 1: {}", part_1(input.clone()));
	println!("Part 2: {}", part_2(input));
}

fn part_1(mut input: Vec<Vec<i64>>) -> i64 {
	(0..1000).for_each(|_| simulation_step(&mut input));
	input.iter().fold(0i64, |acc, x| {
		acc + (x[0..3].iter().map(|v| v.abs()).sum::<i64>()
			* x[3..6].iter().map(|v| v.abs()).sum::<i64>())
	})
}

fn part_2(mut input: Vec<Vec<i64>>) -> i64 {
	let (mut x_loop, mut y_loop, mut z_loop) = (0, 0, 0);
	for step in 1i64.. {
		simulation_step(&mut input);
		if x_loop == 0 && input.iter().all(|x| x[3] == 0) {
			x_loop = step;
		}
		if y_loop == 0 && input.iter().all(|x| x[4] == 0) {
			y_loop = step;
		}
		if z_loop == 0 && input.iter().all(|x| x[5] == 0) {
			z_loop = step;
		}
		if x_loop != 0 && y_loop != 0 && z_loop != 0 {
			break;
		}
	}

	let mut result = x_loop;
	result = (y_loop * result) / (gcd(y_loop, result));
	result = (z_loop * result) / (gcd(z_loop, result));
	result * 2
}

fn simulation_step(input: &mut Vec<Vec<i64>>) {
	for (i, i1) in (0..input.len()).tuple_combinations() {
		for j in 0..3 {
			if input[i][j] < input[i1][j] {
				input[i][j + 3] += 1;
				input[i1][j + 3] -= 1;
			} else if input[i][j] > input[i1][j] {
				input[i][j + 3] -= 1;
				input[i1][j + 3] += 1;
			}
		}
	}
	for i in 0..input.len() {
		for j in 0..3 {
			input[i][j] += input[i][j + 3];
		}
	}
}

fn gcd(a: i64, b: i64) -> i64 {
	if b == 0 {
		a
	} else {
		gcd(b, a % b)
	}
}
