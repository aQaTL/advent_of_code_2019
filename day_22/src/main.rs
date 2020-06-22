use crate::Mode::*;

fn main() {
	let input = std::fs::read_to_string(concat!(env!("CARGO_PKG_NAME"), "/input.txt")).unwrap();
	let input = "cut 6
deal with increment 7
deal into new stack"
		.lines()
		.map(|line| {
			if line.starts_with("deal into new stack") {
				DealIntoNewStack
			} else if line.starts_with("deal with increment") {
				DealWithInc(line[20..].parse::<usize>().unwrap())
			} else if line.starts_with("cut") {
				let x = line[4..].parse::<i32>().unwrap();
				if x < 0 {
					CutFromBot(x.abs() as usize)
				} else {
					CutFromTop(x.abs() as usize)
				}
			} else {
				panic!(format!("Unknown technique: {}", line));
			}
		})
		.collect::<Vec<Mode>>();

	//	println!("Part 1: {}", part_1(&input));

	//	let deck_size = 119315717514047_u64;
	//	let repeat_count = 101741582076661_u64;

	//expected out is 7
	let idx = 2;
	let deck_size = 10;

	for mode in input {
		match mode {
			CutFromBot(cut) | CutFromTop(cut) => {}
		}
	}
}

fn part_1(input: &Vec<Mode>) -> usize {
	let mut deck = (0u16..10_007).collect::<Vec<u16>>();
	for mode in input.iter().cloned() {
		match mode {
			DealIntoNewStack => deck.reverse(),
			DealWithInc(inc) => {
				let mut new_deck = vec![0u16; deck.len()];
				let mut idx = 0;
				for i in 0..new_deck.len() {
					new_deck[idx] = deck[i];
					idx = (idx + inc) % deck.len();
				}
				deck = new_deck;
			}
			CutFromTop(amount) => {
				let mut drain = deck.drain(0..amount).collect::<Vec<_>>();
				deck.append(&mut drain);
			}
			CutFromBot(amount) => {
				let mut drain = deck.drain((deck.len() - amount)..).collect::<Vec<_>>();
				drain.append(&mut deck);
				deck = drain;
			}
		}
	}
	deck.iter()
		.enumerate()
		.find(|(_, el)| **el == 2019)
		.unwrap()
		.0
}

#[derive(Copy, Clone)]
enum Mode {
	DealIntoNewStack,
	DealWithInc(usize),
	CutFromTop(usize),
	CutFromBot(usize),
}
