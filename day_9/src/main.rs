use std::sync::mpsc::{channel, Receiver, Sender};

fn main() {
	let input = std::fs::read_to_string("day_9/input.txt").unwrap();
	let mut input = input
		.split(",")
		.map(|e| e.parse::<i64>().unwrap())
		.collect::<Vec<_>>();

	input.resize(100_000_000, 0);
	println!("Part 1: {}", part_1(input.clone()));
	println!("Part 2: {}", part_2(input));
}

fn part_1(input: Vec<i64>) -> i64 {
	let (sender, receiver) = channel();
	sender.send(1).unwrap();
	solve(input.clone(), sender, receiver)
}

fn part_2(input: Vec<i64>) -> i64 {
	let (sender, receiver) = channel();
	sender.send(2).unwrap();
	solve(input.clone(), sender, receiver)
}

fn solve(mut input: Vec<i64>, sender: Sender<i64>, receiver: Receiver<i64>) -> i64 {
	let mut i = 0;
	let mut prog_output: i64 = 0;
	let mut base: i64 = 0;
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
				let (mut input_1, mut input_2, mut output) =
					(input[i + 1], input[i + 2], input[i + 3]);
				if *params.get(0).unwrap_or(&48) == 48 {
					input_1 = input[input_1 as usize];
				}
				if *params.get(1).unwrap_or(&48) == 48 {
					input_2 = input[input_2 as usize];
				}
				if *params.get(0).unwrap_or(&48) == 50 {
					input_1 = input[(input_1 + base) as usize];
				}
				if *params.get(1).unwrap_or(&48) == 50 {
					input_2 = input[(input_2 + base) as usize];
				}
				if *params.get(2).unwrap_or(&48) == 50 {
					output += base;
				}
				input[output as usize] = input_1 + input_2;
				i += 4;
			}
			"20" | "2" => {
				let (mut input_1, mut input_2, mut output) =
					(input[i + 1], input[i + 2], input[i + 3]);
				if *params.get(0).unwrap_or(&48) == 48 {
					input_1 = input[input_1 as usize];
				}
				if *params.get(1).unwrap_or(&48) == 48 {
					input_2 = input[input_2 as usize];
				}
				if *params.get(0).unwrap_or(&48) == 50 {
					input_1 = input[(input_1 + base) as usize];
				}
				if *params.get(1).unwrap_or(&48) == 50 {
					input_2 = input[(input_2 + base) as usize];
				}
				if *params.get(2).unwrap_or(&48) == 50 {
					output += base;
				}
				input[output as usize] = input_1 * input_2;

				i += 4;
			}
			"30" | "3" => {
				let mut input_1 = input[i + 1];
				if *params.get(0).unwrap_or(&48) == 50 {
					input_1 += base;
				}
				input[input_1 as usize] = receiver.recv().unwrap();
				i += 2;
			}
			"40" | "4" => {
				let mut input_1 = input[i + 1];
				if *params.get(0).unwrap_or(&48) == 48 {
					input_1 = input[input_1 as usize];
				}
				if *params.get(0).unwrap_or(&48) == 50 {
					input_1 = input[(input_1 + base) as usize];
				}
				match sender.send(input_1) {
					_ => (),
				}
				prog_output = input_1;
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
				if *params.get(0).unwrap_or(&48) == 50 {
					input_1 = input[(input_1 + base) as usize];
				}
				if *params.get(1).unwrap_or(&48) == 50 {
					input_2 = input[(input_2 + base) as usize];
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
				if *params.get(0).unwrap_or(&48) == 50 {
					input_1 = input[(input_1 + base) as usize];
				}
				if *params.get(1).unwrap_or(&48) == 50 {
					input_2 = input[(input_2 + base) as usize];
				}
				if input_1 == 0 {
					i = input_2 as usize;
				} else {
					i += 3;
				}
			}
			"70" | "7" => {
				let (mut input_1, mut input_2, mut output) =
					(input[i + 1], input[i + 2], input[i + 3]);
				if *params.get(0).unwrap_or(&48) == 48 {
					input_1 = input[input_1 as usize];
				}
				if *params.get(1).unwrap_or(&48) == 48 {
					input_2 = input[input_2 as usize];
				}
				if *params.get(0).unwrap_or(&48) == 50 {
					input_1 = input[(input_1 + base) as usize];
				}
				if *params.get(1).unwrap_or(&48) == 50 {
					input_2 = input[(input_2 + base) as usize];
				}
				if *params.get(2).unwrap_or(&48) == 50 {
					output += base;
				}
				input[output as usize] = if input_1 < input_2 { 1 } else { 0 };
				i += 4;
			}
			"80" | "8" => {
				let (mut input_1, mut input_2, mut output) =
					(input[i + 1], input[i + 2], input[i + 3]);
				if *params.get(0).unwrap_or(&48) == 48 {
					input_1 = input[input_1 as usize];
				}
				if *params.get(1).unwrap_or(&48) == 48 {
					input_2 = input[input_2 as usize];
				}
				if *params.get(0).unwrap_or(&48) == 50 {
					input_1 = input[(input_1 + base) as usize];
				}
				if *params.get(1).unwrap_or(&48) == 50 {
					input_2 = input[(input_2 + base) as usize];
				}
				if *params.get(2).unwrap_or(&48) == 50 {
					output += base;
				}
				input[output as usize] = if input_1 == input_2 { 1 } else { 0 };
				i += 4;
			}
			"90" | "9" => {
				let mut input_1 = input[i + 1];
				if *params.get(0).unwrap_or(&48) == 48 {
					input_1 = input[input_1 as usize];
				}
				if *params.get(0).unwrap_or(&48) == 50 {
					input_1 = input[(input_1 + base) as usize];
				}
				base += input_1;
				i += 2;
			}
			"99" => break,
			_ => (),
		}
	}

	prog_output
}
