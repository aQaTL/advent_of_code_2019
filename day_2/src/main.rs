fn main() {
	let input = std::fs::read_to_string("day_2/input.txt")
		.unwrap()
		.split(",")
		.map(|e| e.parse::<i64>().unwrap())
		.collect::<Vec<_>>();

	println!("Part 1: {}", solve(input.clone(), 12, 2));

	'loop_: for i in 0..=99 {
		for j in 0..=99 {
			if solve(input.clone(), i, j) == 19690720 {
				println!("Part 2: {}", 100 * i + j);
				break 'loop_;
			}
		}
	}
}

fn solve(mut input: Vec<i64>, i: i64, j: i64) -> i64 {
	input[1] = i;
	input[2] = j;

	for i in (0..).step_by(4) {
		let (input_1, input_2, output) = (
			input[i + 1] as usize,
			input[i + 2] as usize,
			input[i + 3] as usize,
		);
		match input[i] {
			1 => input[output] = input[input_1] + input[input_2],
			2 => input[output] = input[input_1] * input[input_2],
			_ => break,
		}
	}

	input[0]
}
