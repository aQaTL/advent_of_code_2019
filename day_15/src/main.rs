use intcode::Intcode;
use std::collections::{HashMap, HashSet};

fn main() {
	let interpreter = Intcode::parse_input("day_15/input.txt").unwrap();

	let mut grid = HashMap::<(i64, i64), (u8, i64)>::new();

	let mut queue = Vec::<((i64, i64), Intcode, i64)>::new();
	let mut visited = HashSet::<(i64, i64)>::new();

	let start = (0_i64, 0_i64);
	let mut nodes_left_in_layer = 1;
	let mut nodes_in_next_layer = 0;

	queue.push((start, interpreter, 0));
	visited.insert(start);
	grid.insert(start, (1, 0));
	let (mut oxygen_steps, mut oxygen_pos) = (None, (0, 0));

	while let Some((pos, interpreter, move_count)) = queue.pop() {
		for i in 1..=4_i64 {
			let mut interpreter = interpreter.clone();
			let pos = move_point(pos, i);
			let status = interpreter.run_till_output(&[i]).unwrap();
			grid.insert(pos, (status as u8, move_count + 1));
			if status == 2 && oxygen_steps.is_none() {
				oxygen_steps = Some(move_count + 1);
				oxygen_pos = pos;
			}
			match status {
				0 => {
					visited.insert(pos);
				}
				1 | 2 if !visited.contains(&pos) => {
					visited.insert(pos);
					queue.push((pos, interpreter, move_count + 1));
					nodes_in_next_layer += 1;
				}
				_ => (),
			}
		}

		nodes_left_in_layer -= 1;
		if nodes_left_in_layer == 0 {
			nodes_left_in_layer = nodes_in_next_layer;
			nodes_in_next_layer = 0;
		}
	}

	println!("Part 1: {}", oxygen_steps.unwrap());

	let mut queue = Vec::<((i64, i64), i64)>::new();
	visited.clear();
	nodes_left_in_layer = 1;
	nodes_in_next_layer = 0;
	queue.push((oxygen_pos, 0));
	visited.insert(oxygen_pos);
	grid.iter_mut().for_each(|(_, v)| v.1 = -1);

	while let Some((pos, move_count)) = queue.pop() {
		for i in 1..=4_i64 {
			let pos = move_point(pos, i);

			let entry = grid.get_mut(&pos).unwrap();
			if entry.1 == -1 {
				entry.1 = move_count + 1;
			}
			match entry.0 {
				0 => {
					visited.insert(pos);
				}
				1 | 2 if !visited.contains(&pos) => {
					visited.insert(pos);
					queue.push((pos, move_count + 1));
					nodes_in_next_layer += 1;
				}
				_ => (),
			}
		}

		nodes_left_in_layer -= 1;
		if nodes_left_in_layer == 0 {
			nodes_left_in_layer = nodes_in_next_layer;
			nodes_in_next_layer = 0;
		}
	}

	println!("Part 2: {}", grid.values().max_by_key(|v| v.1).unwrap().1);
}

fn move_point(pos: (i64, i64), dir: i64) -> (i64, i64) {
	match dir {
		1 => (pos.0, pos.1 + 1),
		2 => (pos.0, pos.1 - 1),
		3 => (pos.0 - 1, pos.1),
		4 => (pos.0 + 1, pos.1),
		_ => panic!(format!("Invalid dir: {}", dir)),
	}
}
