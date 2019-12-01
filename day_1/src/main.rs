#[rustfmt::skip]
fn main() {
	let input = std::fs::read_to_string("day_1/input.txt")
		.unwrap()
		.lines()
		.map(|e| e.parse::<f32>().unwrap())
		.collect::<Vec<_>>();

	let calc_fuel = |e: f32| (e / 3.0f32).floor() - 2.0;

	println!("Part 1: {}", input.iter().cloned().map(calc_fuel).sum::<f32>());

	let part_2 = input
		.into_iter()
		.fold(0.0, |mut acc, mut mass| {
			while { mass = calc_fuel(mass); mass } > 0.0 {
				acc += mass;
			}
			acc
		});

	println!("Part 2: {}", part_2);
}
