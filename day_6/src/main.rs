use std::collections::HashMap;

fn main() {
	let input = std::fs::read_to_string("day_6/input.txt").unwrap();

	let mut graph = HashMap::new();
	for line in input.lines() {
		let mut s = line.split(")");
		let (a, b) = (s.next().unwrap(), s.next().unwrap());
		graph.entry(a).or_insert(Vec::new()).push(b);
	}

	println!("Part 1: {}", part_1(0, &graph, "COM"));
	println!("Part 2: {}", part_2(&graph, "YOU"));
}

fn part_1(orbits: i32, graph: &HashMap<&str, Vec<&str>>, key: &str) -> i32 {
	match graph.get(key) {
		Some(vec) => {
			orbits
				+ vec
					.iter()
					.map(|o| part_1(orbits + 1, &graph, o))
					.sum::<i32>()
		}
		None => orbits,
	}
}

fn part_2(graph: &HashMap<&str, Vec<&str>>, key: &str) -> i32 {
	let mut current_orbit = graph
		.iter()
		.find(|(_, nodes)| nodes.iter().find(|&&node| node == key).is_some())
		.unwrap();

	let mut distance = 0;
	loop {
		for o in current_orbit.1.iter() {
			let f = dfs(distance + 1, graph, o);
			if f != 0 {
				return f;
			}
		}
		current_orbit = match graph
			.iter()
			.find(|(_, nodes)| nodes.iter().find(|&node| node == current_orbit.0).is_some())
		{
			Some(y) => y,
			None => return 0,
		};
		distance += 1;
	}
}

fn dfs(distance: i32, graph: &HashMap<&str, Vec<&str>>, key: &str) -> i32 {
	let vec = graph.get(key);
	if vec.is_none() {
		return 0;
	}
	for o in vec.unwrap() {
		if *o == "SAN" {
			return distance;
		}
		let f = dfs(distance + 1, &graph, o);
		if f != 0 {
			return f;
		}
	}
	return 0;
}
