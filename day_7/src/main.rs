use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn main() {
	let input = std::fs::read_to_string("day_7/input.txt")
		.unwrap()
		.split(",")
		.map(|e| e.parse::<i64>().unwrap())
		.collect::<Vec<_>>();

	println!("Part 1: {}", part_1(input.clone()));
	println!("Part 2: {}", part_2(input));
}

fn part_1(input: Vec<i64>) -> i64 {
	let mut output = 0;
	for p1 in 0..=4 {
		for p2 in 0..=4 {
			if p2 == p1 {
				continue;
			}
			for p3 in 0..=4 {
				if p2 == p3 || p3 == p1 {
					continue;
				}
				for p4 in 0..=4 {
					if p3 == p4 || p4 == p1 || p4 == p2 {
						continue;
					}
					for p5 in 0..=4 {
						if p5 == p4 || p5 == p3 || p5 == p2 || p5 == p1 {
							continue;
						}
						let mut inp = 0;
						for &phase in [p1, p2, p3, p4, p5].iter() {
							let (sender, receiver) = channel();
							sender.send(phase).unwrap();
							sender.send(inp).unwrap();
							inp = solve(input.clone(), sender, receiver);
						}
						if inp > output {
							output = inp;
						}
					}
				}
			}
		}
	}
	output
}

fn part_2(input: Vec<i64>) -> i64 {
	let mut output = 0;
	for p1 in 5..=9 {
		for p2 in 5..=9 {
			if p2 == p1 {
				continue;
			}
			for p3 in 5..=9 {
				if p2 == p3 || p3 == p1 {
					continue;
				}
				for p4 in 5..=9 {
					if p3 == p4 || p4 == p1 || p4 == p2 {
						continue;
					}
					for p5 in 5..=9 {
						if p5 == p4 || p5 == p3 || p5 == p2 || p5 == p1 {
							continue;
						}
						let (sender1, receiver1) = channel();
						let (sender2, receiver2) = channel();
						let (sender3, receiver3) = channel();
						let (sender4, receiver4) = channel();
						let (sender5, receiver5) = channel();
						sender1.send(p1).unwrap();
						sender2.send(p2).unwrap();
						sender3.send(p3).unwrap();
						sender4.send(p4).unwrap();
						sender5.send(p5).unwrap();
						sender1.send(0).unwrap();

						let input_c = input.clone();
						thread::spawn(move || solve(input_c, sender2, receiver1));
						let input_c = input.clone();
						thread::spawn(move || solve(input_c, sender3, receiver2));
						let input_c = input.clone();
						thread::spawn(move || solve(input_c, sender4, receiver3));
						let input_c = input.clone();
						thread::spawn(move || solve(input_c, sender5, receiver4));
						let prog_output = solve(input.clone(), sender1, receiver5);
						if prog_output > output {
							output = prog_output;
						}
					}
				}
			}
		}
	}
	output
}

fn solve(mut input: Vec<i64>, sender: Sender<i64>, receiver: Receiver<i64>) -> i64 {
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
				input[output] = receiver.recv().unwrap();
				i += 2;
			}
			"40" | "4" => {
				match sender.send(input[input[i + 1] as usize]) {
					_ => (),
				}
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
