fn main() {
	let input = std::fs::read_to_string("day_2/input.txt")
		.unwrap()
		.split(",")
		.map(|e| e.parse::<i64>().unwrap())
		.collect::<Vec<_>>();

	println!("Part 1: {}", solve(input.clone(), 12, 2));

	for i in 0..=99 {
		for j in 0..=99 {
			let result = solve(input.clone(), i, j);
			if result == 19690720 {
				println!("Part 2: {}", 100 * i + j);
				break;
			}
		}
	}
}

fn solve(mut input: Vec<i64>, i: i64, j: i64) -> i64 {
	input[1] = i;
	input[2] = j;

	let mut i = 0;
	loop {
		let (input_1, input_2, output) = (
			input[i + 1] as usize,
			input[i + 2] as usize,
			input[i + 3] as usize,
		);
		match input[i] {
			1 => input[output] = input[input_1] + input[input_2],
			2 => input[output] = input[input_1] * input[input_2],
			99 => break,
			_ => (),
		}
		i += 4;
	}

	input[0]
}
