use std::collections::HashMap;

fn main() {
	let input = std::fs::read_to_string("day_4/input.txt").unwrap();
	let mut input = input.split("-");
	let from = input.next().unwrap().parse::<u64>().unwrap();
	let to = input.next().unwrap().parse::<u64>().unwrap();

	let (mut part_1, mut part_2) = (0, 0);
	let mut hm: HashMap<u8, u8> = HashMap::new();
	'pass_: for i in from..=to {
		hm.clear();
		for w in i.to_string().as_bytes().windows(2) {
			if w[0] > w[1] {
				continue 'pass_;
			}
			if w[0] == w[1] {
				*hm.entry(w[0]).or_insert(1) += 1;
			}
		}
		if hm.values().find(|&&e| e >= 2).is_some() {
			part_1 += 1;
		}
		if hm.values().find(|&&e| e == 2).is_some() {
			part_2 += 1;
		}
	}
	println!("Part 1: {}", part_1);
	println!("Part 2: {}", part_2);
}
