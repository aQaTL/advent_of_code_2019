use intcode::Intcode;
use std::sync::mpsc::channel;

fn main() {
	let input = Intcode::new(Intcode::parse_input("day_9/input.txt").unwrap());

	println!("Part 1: {}", part_1(input.clone()));
	println!("Part 2: {}", part_2(input));
}

fn part_1(mut input: Intcode) -> i64 {
	let (sender, receiver) = channel();
	sender.send(1).unwrap();
	input.run(sender, receiver)
}

fn part_2(mut input: Intcode) -> i64 {
	let (sender, receiver) = channel();
	sender.send(2).unwrap();
	input.run(sender, receiver)
}
