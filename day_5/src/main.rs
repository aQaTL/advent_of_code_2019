fn main() {
	let input = std::fs::read_to_string("day_5/input.txt")
		.unwrap()
		.split(",")
		.map(|e| e.parse::<i64>().unwrap())
		.collect::<Vec<_>>();

	println!("Part 1: {}", solve(input.clone(), 1));
	println!("Part 2: {}", solve(input, 5));
}

fn solve(mut input: Vec<i64>, id: i64) -> i64 {
	let mut i = 0;
	let mut prog_output = 0;
	loop {
		let mut inp = input[i].to_string();
		let inp = unsafe { inp.as_bytes_mut() };
		inp.reverse();
		let (op, params) = if inp.len() >= 2 {
			inp.split_at_mut(2)
		} else {
			(inp, &mut [][..])
		};
		match std::str::from_utf8(op).unwrap() {
			"10" | "1" => {
				let (mut input_1, mut input_2, output) =
					(input[i + 1], input[i + 2], input[i + 3] as usize);
				if *params.get(0).unwrap_or(&48) == 48 {
					input_1 = input[input_1 as usize];
				}
				if *params.get(1).unwrap_or(&48) == 48 {
					input_2 = input[input_2 as usize];
				}
				input[output] = input_1 + input_2;
				i += 4;
			}
			"20" | "2" => {
				let (mut input_1, mut input_2, output) =
					(input[i + 1], input[i + 2], input[i + 3] as usize);
				if *params.get(0).unwrap_or(&48) == 48 {
					input_1 = input[input_1 as usize];
				}
				if *params.get(1).unwrap_or(&48) == 48 {
					input_2 = input[input_2 as usize];
				}
				input[output] = input_1 * input_2;

				i += 4;
			}
			"3" => {
				let output = input[i + 1] as usize;
				input[output] = id;
				i += 2;
			}
			"40" | "4" => {
				prog_output = input[input[i + 1] as usize];
				i += 2;
			}
			"50" | "5" => {
				let (mut input_1, mut input_2) = (input[i + 1], input[i + 2]);
				if *params.get(0).unwrap_or(&48) == 48 {
					input_1 = input[input_1 as usize];
				}
				if *params.get(1).unwrap_or(&48) == 48 {
					input_2 = input[input_2 as usize];
				}
				if input_1 != 0 {
					i = input_2 as usize;
				} else {
					i += 3;
				}
			}
			"60" | "6" => {
				let (mut input_1, mut input_2) = (input[i + 1], input[i + 2]);
				if *params.get(0).unwrap_or(&48) == 48 {
					input_1 = input[input_1 as usize];
				}
				if *params.get(1).unwrap_or(&48) == 48 {
					input_2 = input[input_2 as usize];
				}
				if input_1 == 0 {
					i = input_2 as usize;
				} else {
					i += 3;
				}
			}
			"70" | "7" => {
				let (mut input_1, mut input_2, output) =
					(input[i + 1], input[i + 2], input[i + 3] as usize);
				if *params.get(0).unwrap_or(&48) == 48 {
					input_1 = input[input_1 as usize];
				}
				if *params.get(1).unwrap_or(&48) == 48 {
					input_2 = input[input_2 as usize];
				}
				input[output] = if input_1 < input_2 { 1 } else { 0 };
				i += 4;
			}
			"80" | "8" => {
				let (mut input_1, mut input_2, output) =
					(input[i + 1], input[i + 2], input[i + 3] as usize);
				if *params.get(0).unwrap_or(&48) == 48 {
					input_1 = input[input_1 as usize];
				}
				if *params.get(1).unwrap_or(&48) == 48 {
					input_2 = input[input_2 as usize];
				}
				input[output] = if input_1 == input_2 { 1 } else { 0 };
				i += 4;
			}
			"99" => break,
			_ => (),
		}
	}

	prog_output
}
