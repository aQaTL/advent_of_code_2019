use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn main() {
	let input = std::fs::read_to_string("day_11/input.txt").unwrap();
	let mut input = input
		.split(",")
		.map(|e| e.parse::<i64>().unwrap())
		.collect::<Vec<_>>();

	input.resize(100_000_000, 0);
	part_1(input.clone());
	part_2(input);
}

fn part_1(input: Vec<i64>) {
	let mut grid = HashMap::<(i64, i64), (i64, i64)>::new();
	run_painter(input, &mut grid);
	let result = grid.values().filter(|(_, visited)| *visited > 0).count();
	println!("Part 1: {}", result);
}

fn part_2(input: Vec<i64>) {
	let mut grid = HashMap::<(i64, i64), (i64, i64)>::new();
	grid.insert((0, 0), (1, 0));
	run_painter(input, &mut grid);

	let lowest_x = grid.keys().min_by_key(|k| k.0).unwrap().0;
	let lowest_y = grid.keys().min_by_key(|k| k.1).unwrap().1;
	let highest_x = grid.keys().max_by_key(|k| k.0).unwrap().0;
	let highest_y = grid.keys().max_by_key(|k| k.1).unwrap().1;

	println!("Part 2:");
	for j in (lowest_y..=highest_y).rev() {
		for i in lowest_x..=highest_x {
			if grid.get(&(i, j)).unwrap_or(&(0, 0)).0 == 1 {
				print!("\u{2588}");
			} else {
				print!(" ");
			}
		}
		println!();
	}
}

//HashMap<(x, y), (color, visited)>
fn run_painter(input: Vec<i64>, grid: &mut HashMap<(i64, i64), (i64, i64)>) {
	let (sender, receiver) = channel();
	let (sender2, receiver2) = channel();
	thread::spawn(move || solve(input, sender2, receiver));

	let mut pos = (0i64, 0i64);
	let mut dir = 0;

	loop {
		let c = grid.entry(pos).or_insert((0, 0)).0;
		match sender.send(c) {
			_ => (),
		}
		match receiver2.recv() {
			Ok(new_color) => match receiver2.recv() {
				Ok(new_dir) => {
					let entry = grid.entry(pos).or_insert((0, 0));
					entry.0 = new_color;
					entry.1 += 1;
					dir = match new_dir {
						0 => {
							let n = dir - 1;
							if n < 0 {
								3
							} else {
								n
							}
						}
						1 => (dir + 1) % 4,
						_ => unreachable!(),
					};
				}
				Err(_) => break,
			},
			Err(_) => break,
		}
		match dir {
			0 => pos.1 += 1,
			1 => pos.0 += 1,
			2 => pos.1 -= 1,
			3 => pos.0 -= 1,
			_ => unreachable!(),
		}
	}
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
				let i1 = receiver.recv().unwrap();
				input[input_1 as usize] = i1;
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
