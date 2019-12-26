use std::collections::{HashMap, HashSet, VecDeque};
use Tile::*;

fn main() {
	let input = std::fs::read_to_string(concat!(env!("CARGO_PKG_NAME"), "/input.txt")).unwrap();
	let mut maze = HashMap::<(i64, i64), char>::new();
	let mut pos = (0, 0);
	for c in input.chars() {
		match c {
			'A'..='Z' | '.' => {
				maze.insert(pos, c);
				pos.0 += 1;
			}
			'\n' => pos = (0, pos.1 + 1),
			_ => pos.0 += 1,
		}
	}
	let width = maze.keys().max_by_key(|(x, _)| x).unwrap().0;
	let height = maze.keys().max_by_key(|(_, y)| y).unwrap().1;

	let mut maze2 = HashMap::<(i64, i64), Tile>::new();
	let mut portals = HashMap::new();
	for (k, v) in maze.iter() {
		let (k, v) = (*k, *v);
		match v {
			'A'..='Z' => {
				for dir in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
					let new_pos = (k.0 + dir.0, k.1 + dir.1);
					match maze.get(&new_pos) {
						Some(c2 @ 'A'..='Z') => {
							if new_pos.0 < k.0 || new_pos.1 < k.1 {
								continue;
							}
							let (entry, tel_pos) = {
								let mut ret = Default::default();
								for pos in &[k, new_pos] {
									if let Some(p) = (&[(0, -1), (1, 0), (0, 1), (-1, 0)])
										.iter()
										.map(|d| (pos.0 + d.0, pos.1 + d.1))
										.find_map(|p| match maze.get(&p) {
											Some('.') => Some((p, *pos)),
											_ => None,
										}) {
										ret = p;
										break;
									}
								}
								ret
							};
							maze2.insert(k, Portal((v, *c2)));
							maze2.insert(new_pos, Portal((v, *c2)));
							portals
								.entry((v, *c2))
								.or_insert(Vec::new())
								.push(Teleport {
									pos: tel_pos,
									entrance: entry,
									ttype: if entry.0 == 2
										|| entry.1 == 2 || entry.0 == width - 2 || entry.1
										== height - 2
									{
										TeleportType::Outer
									} else {
										TeleportType::Inner
									},
								});
							break;
						}
						_ => (),
					}
				}
			}
			'.' => {
				maze2.insert(k, Open);
			}
			_ => (),
		}
	}
	let maze = maze2;

	part_1(&portals, &maze);
	part_2(&portals, &maze);
}

fn part_2(portals: &HashMap<(char, char), Vec<Teleport>>, maze: &HashMap<(i64, i64), Tile>) {
	let mut queue = VecDeque::<((i64, i64), i64)>::new();
	let mut visited = HashMap::<i64, HashSet<(i64, i64)>>::new();
	let start = portals[&('A', 'A')][0].entrance;
	queue.push_back((start, 0));
	visited.insert(0, {
		let mut hs = HashSet::new();
		hs.insert(start);
		hs
	});
	let mut nodes_left_in_layer = 1;
	let mut nodes_in_next_layer = 0;
	let mut moves_count = 0;
	'part_2: while let Some((pos, level)) = queue.pop_front() {
		for dir in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
			let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
			let new_tile = match maze.get(&new_pos) {
				Some(t) => t,
				None => continue,
			};
			if !visited
				.entry(level)
				.or_insert(HashSet::new())
				.insert(new_pos)
			{
				continue;
			}

			match new_tile {
				Open => {
					queue.push_back((new_pos, level));
					visited
						.entry(level)
						.or_insert(HashSet::new())
						.insert(new_pos);
					nodes_in_next_layer += 1;
				}
				Portal(('A', 'A')) => continue,
				Portal(name) => {
					if name.0 == 'Z' && name.1 == 'Z' {
						if level == 0 {
							break 'part_2;
						} else {
							continue;
						}
					}
					let teleport_pos = portals[name].iter().find(|x| x.pos != new_pos).unwrap();
					match teleport_pos.ttype {
						TeleportType::Outer => {
							queue.push_back((teleport_pos.entrance, level + 1));
							let visited_n = visited.entry(level + 1).or_insert(HashSet::new());
							visited_n.insert(teleport_pos.entrance);
							visited_n.insert(teleport_pos.pos);
						}
						TeleportType::Inner => {
							if level == 0 {
								continue;
							}
							queue.push_back((teleport_pos.entrance, level - 1));
							let visited_n = visited.entry(level - 1).or_insert(HashSet::new());
							visited_n.insert(teleport_pos.entrance);
							visited_n.insert(teleport_pos.pos);
						}
					}
					nodes_in_next_layer += 1;
				}
			}
		}
		nodes_left_in_layer -= 1;
		if nodes_left_in_layer == 0 {
			nodes_left_in_layer = nodes_in_next_layer;
			nodes_in_next_layer = 0;
			moves_count += 1;
		}
	}
	println!("Part 2: {}", moves_count);
}

fn part_1(portals: &HashMap<(char, char), Vec<Teleport>>, maze: &HashMap<(i64, i64), Tile>) {
	let mut queue = VecDeque::<(i64, i64)>::new();
	let mut visited = HashSet::<(i64, i64)>::new();
	let start = portals[&('A', 'A')][0].entrance;
	queue.push_back(start);
	visited.insert(start);
	let mut nodes_left_in_layer = 1;
	let mut nodes_in_next_layer = 0;
	let mut moves_count = 0;
	'part_1: while let Some(pos) = queue.pop_front() {
		for dir in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
			let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
			let new_tile = match maze.get(&new_pos) {
				Some(t) => t,
				None => continue,
			};
			if !visited.insert(new_pos) {
				continue;
			}

			match new_tile {
				Open => {
					queue.push_back(new_pos);
					visited.insert(new_pos);
					nodes_in_next_layer += 1;
				}
				Portal(('A', 'A')) => continue,
				Portal(name) => {
					if name.0 == 'Z' && name.1 == 'Z' {
						break 'part_1;
					}
					let teleport_pos = portals[name].iter().find(|x| x.pos != new_pos).unwrap();
					queue.push_back(teleport_pos.entrance);
					visited.insert(teleport_pos.entrance);
					visited.insert(teleport_pos.pos);
					nodes_in_next_layer += 1;
				}
			}
		}
		nodes_left_in_layer -= 1;
		if nodes_left_in_layer == 0 {
			nodes_left_in_layer = nodes_in_next_layer;
			nodes_in_next_layer = 0;
			moves_count += 1;
		}
	}
	println!("Part 1: {}", moves_count);
}

enum Tile {
	Open,
	Portal((char, char)),
}

struct Teleport {
	pos: (i64, i64),
	entrance: (i64, i64),
	ttype: TeleportType,
}

enum TeleportType {
	Inner,
	Outer,
}
