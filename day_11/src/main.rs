use intcode::Intcode;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
	let input = std::fs::read_to_string("day_11/input.txt").unwrap();
	let input = input
		.split(",")
		.enumerate()
		.map(|(idx, e)| (idx as i64, e.parse::<i64>().unwrap()))
		.collect::<HashMap<i64, i64>>();
	let intcode = Intcode::new(input);

	part_1(intcode.clone());
	part_2(intcode);
}

fn part_1(intcode: Intcode) {
	let mut grid = HashMap::<(i64, i64), (i64, i64)>::new();
	run_painter(intcode, &mut grid);
	let result = grid.values().filter(|(_, visited)| *visited > 0).count();
	println!("Part 1: {}", result);
}

fn part_2(intcode: Intcode) {
	let mut grid = HashMap::<(i64, i64), (i64, i64)>::new();
	grid.insert((0, 0), (1, 0));
	run_painter(intcode, &mut grid);

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
fn run_painter(mut intcode: Intcode, grid: &mut HashMap<(i64, i64), (i64, i64)>) {
	let (sender, receiver) = channel();
	let (sender2, receiver2) = channel();
	thread::spawn(move || intcode.run(sender2, receiver));

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
