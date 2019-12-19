/// Rewrite using idea from https://todd.ginsberg.com/post/advent-of-code/2019/day14/
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
	let input = std::fs::read_to_string("day_14/input.txt").unwrap();
	let reactions = input
		.lines()
		.map(|line| {
			let mut split = line.split(" => ");

			let reactors: Vec<Reagent> = {
				split
					.next()
					.unwrap()
					.split(", ")
					.map(|reactor| {
						let mut split = reactor.split(" ");
						Reagent {
							amount: split.next().unwrap().parse::<i64>().unwrap(),
							name: split.next().unwrap(),
						}
					})
					.collect::<Vec<Reagent>>()
			};
			let produced: (i64, &str) = {
				let mut split = split.next().unwrap().split(" ");
				(
					split.next().unwrap().parse::<i64>().unwrap(),
					split.next().unwrap(),
				)
			};

			(
				produced.1,
				Reaction {
					produced_amount: produced.0,
					ingredient: reactors,
				},
			)
		})
		.collect::<HashMap<&str, Reaction>>();

	println!("Part 1: {}", part_1(&reactions));
	println!("Part 2: {}", part_2(&reactions));
}

fn part_1<'a>(reactions: &HashMap<&'a str, Reaction<'a>>) -> i64 {
	let mut bag = HashMap::<&str, i64>::new();
	let mut ore_count = 0;
	produce(&reactions, &mut bag, "FUEL", 1, &mut ore_count);
	ore_count
}

fn part_2<'a>(reactions: &HashMap<&'a str, Reaction<'a>>) -> i64 {
	let mut bag = HashMap::<&str, i64>::new();
	let goal = 1_000_000_000_000_i64;

	let (mut min, mut max) = (1, goal);

	while min <= max {
		let mid = (min + max) / 2;

		bag.clear();
		let mut ore_count = 0;
		produce(&reactions, &mut bag, "FUEL", mid, &mut ore_count);

		match goal.cmp(&ore_count) {
			Ordering::Less => max = mid - 1,
			Ordering::Greater => min = mid + 1,
			Ordering::Equal => {
				return mid;
			}
		}
	}

	min - 1
}

fn produce<'a>(
	reactions: &HashMap<&'a str, Reaction<'a>>,
	bag: &mut HashMap<&'a str, i64>,
	material: &'a str,
	mut amount: i64,
	ore_count: &mut i64,
) {
	let reaction = &reactions[material];

	let in_bag = bag.entry(material).or_default();

	if *in_bag >= amount {
		*in_bag -= amount;
		return;
	}

	amount -= *in_bag;

	let iterations = (amount as f64 / reaction.produced_amount as f64).ceil() as i64;

	for deps in reaction.ingredient.iter() {
		if deps.name == "ORE" {
			*ore_count += deps.amount * iterations;
			continue;
		}
		let mut dep_iterations = iterations;
		let dep_bag = bag.entry(deps.name).or_default();
		while dep_iterations > 0 && *dep_bag >= deps.amount {
			dep_iterations -= 1;
			*dep_bag -= deps.amount;
		}
		if *dep_bag < (deps.amount * dep_iterations) {
			produce(
				reactions,
				bag,
				deps.name,
				deps.amount * dep_iterations,
				ore_count,
			);
		}
		*bag.entry(deps.name).or_default() -= deps.amount * dep_iterations;
	}
	*bag.entry(material).or_default() += reaction.produced_amount * iterations;
}

struct Reaction<'a> {
	produced_amount: i64,
	ingredient: Vec<Reagent<'a>>,
}

#[derive(Copy, Clone)]
struct Reagent<'a> {
	name: &'a str,
	amount: i64,
}
