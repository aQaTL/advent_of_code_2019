use intcode::Intcode;

fn main() {
	let interpreter = Intcode::new(Intcode::parse_input("day_19/input.txt").unwrap());

	let mut sum = 0;
	for j in 0..50 {
		for i in 0..50 {
			sum += interpreter.clone().run_till_output(&[i, j]).unwrap();
		}
	}
	println!("Part 1: {}", sum);

	let mut pos = (0, 99);
	loop {
		while interpreter
			.clone()
			.run_till_output(&[pos.0, pos.1])
			.unwrap() == 0
		{
			pos.0 += 1;
		}
		if interpreter
			.clone()
			.run_till_output(&[pos.0 + 99, pos.1 - 99])
			.unwrap() == 1
		{
			break;
		} else {
			pos.1 += 1;
		}
	}
	println!("Part 2: {}", pos.0 * 10_000 + pos.1 - 99);
}
