use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn main() {
	let input = std::fs::read_to_string("day_13/input.txt").unwrap();
	let mut input = input
		.split(",")
		.map(|e| e.parse::<i64>().unwrap())
		.collect::<Vec<_>>();

	input.resize(100_000_000, 0);

	let input_c = input.clone();
	let part1_handle = thread::spawn(move || println!("Part 1: {}", part_1(input_c)));
	let part2_handle = thread::spawn(move || println!("Part 2: {}", part_2(input)));
	part1_handle.join().unwrap();
	part2_handle.join().unwrap();
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
enum Tile {
	Empty,
	Wall,
	Block,
	Paddle,
	Ball,
}

fn part_1(input: Vec<i64>) -> usize {
	let mut grid = HashMap::<(i64, i64), Tile>::new();
	let (_, receiver) = channel();
	let (sender2, receiver2) = channel();
	thread::spawn(move || solve(input, sender2, receiver));

	loop {
		let x = match receiver2.recv() {
			Ok(x) => x,
			Err(_) => break,
		};
		let y = match receiver2.recv() {
			Ok(y) => y,
			Err(_) => break,
		};
		let id = match receiver2.recv() {
			Ok(id) => match id {
				0 => Tile::Empty,
				1 => Tile::Wall,
				2 => Tile::Block,
				3 => Tile::Paddle,
				4 => Tile::Ball,
				_ => unreachable!(),
			},
			Err(_) => break,
		};
		grid.insert((x, y), id);
	}

	grid.values().filter(|id| **id == Tile::Block).count()
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

fn part_2(mut input: Vec<i64>) -> i64 {
	input[0] = 2;
	let mut grid = HashMap::<(i64, i64), Tile>::new();

	let mut output_queue = Vec::with_capacity(3);
	let (mut ball_pos, mut paddle_pos) = ((0, 0), (0, 0));
	let mut score: i64 = 0;

	let mut i = 0;
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
				input[input_1 as usize] = {
					match (ball_pos).0.cmp(&(paddle_pos).0) {
						Ordering::Less => -1,
						Ordering::Greater => 1,
						Ordering::Equal => 0,
					}
				};
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
				output_queue.push(input_1);
				if output_queue.len() == 3 {
					match output_queue.as_slice() {
						&[x, y, third] => {
							if x == -1 && y == 0 {
								score = third;
							} else {
								let tile = match third {
									0 => Tile::Empty,
									1 => Tile::Wall,
									2 => Tile::Block,
									3 => {
										paddle_pos = (x, y);
										Tile::Paddle
									}
									4 => {
										ball_pos = (x, y);
										Tile::Ball
									}
									_ => unreachable!(),
								};
								grid.insert((x, y), tile);
							}
							output_queue.clear();
						}
						_ => unreachable!(),
					}
				}
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

	score
}
