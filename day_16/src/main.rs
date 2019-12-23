fn main() {
	let input = std::fs::read_to_string("day_16/input.txt").unwrap();
	part_1(&input);
	part_2(&input);
}

fn part_1(input_str: &str) {
	let mut input = input_str
		.bytes()
		.map(|b| (b - 48) as i64)
		.collect::<Vec<i64>>();
	let mut output = Vec::with_capacity(input.len());
	let pattern = vec![0, 1, 0, -1];

	for _ in 0..100 {
		for i in 0..input.len() {
			let mut repeat_idx = 1;
			let mut pattern_idx = 0;
			let mut sum = 0;
			for &x in &input {
				if repeat_idx >= (i + 1) {
					repeat_idx = 0;
					pattern_idx = (pattern_idx + 1) % pattern.len();
				}
				sum += x * pattern[pattern_idx];
				repeat_idx += 1;
			}
			output.push(sum.abs() % 10);
		}
		input.clear();
		input.append(&mut output);
	}
	println!(
		"Part 1: {}",
		input
			.iter()
			.take(8)
			.map(ToString::to_string)
			.collect::<String>()
	);
}

fn part_2(input_str: &str) {
	let input = input_str
		.bytes()
		.map(|b| (b - 48) as i64)
		.collect::<Vec<i64>>();
	let base_len = input.len();
	let msg_offset = input_str[..7].parse::<usize>().unwrap();
	let mut input = input
		.into_iter()
		.cycle()
		.take(base_len * 10_000)
		.skip(msg_offset)
		.collect::<Vec<_>>();

	let mut output = Vec::with_capacity(input.len());

	for _ in 0..100 {
		output.push(*input.last().unwrap());
		for i in (0..(input.len() - 1)).rev() {
			output.push((*output.last().unwrap() + input[i]) % 10);
		}
		input.clear();
		output.drain(..).rev().for_each(|x| input.push(x));
	}
	println!(
		"Part 2: {}",
		input
			.iter()
			.take(8)
			.map(ToString::to_string)
			.collect::<String>()
	);
}
