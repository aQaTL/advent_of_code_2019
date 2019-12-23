use intcode::Intcode;
use std::sync::mpsc::channel;
use Movement::*;
use itertools::Itertools;

fn main() {
	let input = Intcode::parse_input("day_17/input.txt").unwrap();
	let mut interpreter = Intcode::new(input.clone());
	let (_sender, r) = channel();
	let (s, receiver) = channel();
	std::thread::spawn(move || interpreter.run(s, r));

	let mut img = Vec::new();
	while let Ok(output) = receiver.recv() {
		let output = output as u8 as char;
		img.push(output);
	}

	let width = img
		.iter()
		.enumerate()
		.find(|(_, el)| **el == '\n')
		.unwrap()
		.0 + 1;
	let height = img.len() / width;

	let mut sum = 0;
	for j in 0..height {
		for i in 0..width {
			if img[j * width + i] != '#'
				|| *img.get(j * width + i + 1).unwrap_or(&'.') != '#'
				|| *img.get(j * width + i - 1).unwrap_or(&'.') != '#'
				|| *img.get((j + 1) * width + i).unwrap_or(&'.') != '#'
				|| *img.get((j - 1) * width + i).unwrap_or(&'.') != '#'
			{
				continue;
			}
			sum += j * i;
		}
	}

	println!("Part 1: {}", sum);

	// Finding the route for part 2

	let robot_idx = img.iter().enumerate().find(|(_, x)| **x == '^').unwrap().0;
	let mut robot_pos = ((robot_idx % width) as i64, (robot_idx / height) as i64); //(x, y)
	let mut robot_dir = 0_i64;
	let mut robot_dir_vec = (0, -1);

	let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];

	let mut movements = Vec::<Movement>::new();

	loop {
		let mut steps = 0;
		while get_from_vec(&img,
						   ((robot_pos.1 + robot_dir_vec.1) * (width as i64)) + (robot_pos.0 + robot_dir_vec.0))
			.unwrap_or('.') == '#'
			{
				robot_pos = (robot_pos.0 + robot_dir_vec.0, robot_pos.1 + robot_dir_vec.1);
				steps += 1;
			}
		if steps != 0 {
			movements.push(Movement::Move(steps));
		}

		let new_dir = {
			let mut t = None;
			for (idx, dir_vec) in dirs.iter().enumerate().filter(|(idx, _)| ((*idx + 2) % 4) as i64 != robot_dir) {
				if get_from_vec(&img,
								(robot_pos.1 + dir_vec.1) * (width as i64) + (robot_pos.0 + dir_vec.0))
					.unwrap_or('.')
					== '#'
				{
					t = Some((idx as i64, *dir_vec));
					break;
				}
			}
			t
		};

		if new_dir.is_none() {
			break;
		}
		let new_dir = new_dir.unwrap();

		if { if robot_dir - 1 >= 0 { robot_dir - 1 } else { 3 } } == new_dir.0 {
			movements.push(Movement::Turn('L'));
		} else if (robot_dir + 1) % 4 == new_dir.0 {
			movements.push(Movement::Turn('R'));
		} else {
			panic!(format!("Old dir: {}, new dir: {} at pos {:?}", robot_dir, new_dir.0, robot_pos));
		}

		robot_dir = new_dir.0;
		robot_dir_vec = new_dir.1;
	}

	for mov in movements {
		match mov {
			Movement::Turn(x) => print!("{},", x),
			Movement::Move(x) => print!("{},", x),
		}
	}
	println!();

	// Running part 2

	let mut input = input;
	input.insert(0, 2);
	let mut interpreter = Intcode::new(input);

	let (sender, r) = channel();
	let (s, receiver) = channel();
	std::thread::spawn(move || interpreter.run(s, r));

	let a = [Turn('L'), Move(1), Move(9), Turn('R'), Move(8), Turn('R'), Move(8), Turn('L'), Move(9), Move(1)];
	let b = [Turn('L'), Move(9), Move(1), Turn('L'), Move(6), Turn('R'), Move(9), Move(1)];
	let c = [Turn('R'), Move(6), Turn('R'), Move(8), Turn('R'), Move(8), Turn('L'), Move(6), Turn('R'), Move(8)];

	let move_routine = "B,C,B,A,C,A,C,B,A,C";
	move_routine.as_bytes().iter().for_each(|&x| sender.send(x as i64).unwrap());
	sender.send(b'\n' as i64).unwrap();

	let send = |arr: &[Movement]| {
		arr.iter().interleave([Turn(',')].iter().cycle().take(arr.len() - 1)).for_each(|m| {
			match m {
				Turn(t) => sender.send(*t as u8 as i64).unwrap(),
				Move(steps) => sender.send(*steps + 48).unwrap(),
			}
		});
		sender.send(b'\n' as i64).unwrap();
	};
	send(&a);
	send(&b);
	send(&c);

	sender.send(b'n' as i64).unwrap();
	sender.send(b'\n' as i64).unwrap();
	let mut output = 0;
	while let Ok(out) = receiver.recv() {
		output = out;
	}
	println!("Part 2: {}", output);
}

fn get_from_vec<T: Copy>(vec: &Vec<T>, idx: i64) -> Option<T> {
	if idx < 0 || idx >= vec.len() as i64 {
		None
	} else {
		Some(vec[idx as usize])
	}
}

#[derive(Debug)]
enum Movement {
	Turn(char),
	Move(i64),
}
