use bit_set::BitSet;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
	let input = std::fs::read_to_string("day_18/input.txt").unwrap();
	let mut grid = HashMap::<(i64, i64), char>::new();
	let mut keys = Vec::<char>::new();
	let mut start = (0, 0);
	{
		let (mut x, mut y) = (0, 0);
		for cell in input.chars() {
			if cell == '\n' {
				y += 1;
				x = 0;
				continue;
			} else {
				grid.insert((x, y), cell);
			}

			if cell == '@' {
				start = (x, y);
			} else if cell.is_ascii_lowercase() {
				keys.push(cell);
			}

			x += 1;
		}
	}

	let mut robots = Vec::with_capacity(4);
	{
		for dir in &[(-1, -1), (-1, 1), (1, -1), (-1, 1)] {
			let pos = (start.0 + dir.0, start.1 + dir.1);
			grid.insert(pos, '@');
			robots.push(pos);
		}
		for pos in (0..4).map(|i| move_point(start, i)) {
			grid.insert(pos, '#');
		}
		grid.insert(start, '#');
	}

	let mut queue = VecDeque::<((i64, i64), BitSet)>::new();
	let mut visited = HashMap::<((i64, i64), BitSet), i64>::new();
	visited.insert((start, BitSet::with_capacity(keys.len())), 0);

	queue.push_back((start, BitSet::with_capacity(keys.len())));

	let mut moves_min = None;

	while let Some(state) = queue.pop_front() {
		let steps = match visited.get(&state) {
			Some(s) => *s,
			None => break,
		};

		if state.1.len() == keys.len() {
			moves_min = Some(steps);
			break;
		}

		for i in 0..4 {
			let pos = move_point(state.0, i);

			let c = match grid.get(&pos) {
				Some(c) => *c,
				None => continue,
			};
			if c == '#' {
				continue;
			}
			if c.is_ascii_uppercase() && !state.1.contains((c as u8 - b'A') as usize) {
				continue;
			}

			let mut new_state = (pos, state.1.clone());

			if visited.contains_key(&new_state) {
				continue;
			}

			if c.is_ascii_lowercase() && !new_state.1.contains((c as u8 - b'a') as usize) {
				new_state.1.insert((c as u8 - b'a') as usize);
			}

			visited.insert(new_state.clone(), steps + 1);
			queue.push_back(new_state);
		}
	}

	println!("Part 2: {}", moves_min.unwrap());
}

fn move_point(pos: (i64, i64), dir: i64) -> (i64, i64) {
	match dir {
		0 => (pos.0, pos.1 - 1),
		1 => (pos.0 + 1, pos.1),
		2 => (pos.0, pos.1 + 1),
		3 => (pos.0 - 1, pos.1),
		_ => panic!(format!("Invalid dir: {}", dir)),
	}
}
