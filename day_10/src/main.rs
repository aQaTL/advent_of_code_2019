use std::collections::HashMap;

fn main() {
	let input = std::fs::read_to_string("day_10/input.txt")
		.unwrap()
		.lines()
		.map(|line| line.chars().collect::<Vec<_>>())
		.collect::<Vec<Vec<char>>>();

	let mut points = Vec::new();
	for (j, line) in input.iter().enumerate() {
		for (i, &point) in line.iter().enumerate() {
			if point == '#' {
				points.push((i as f64, j as f64));
			}
		}
	}

	let mut best = 0;
	let mut station_asteroids: HashMap<i64, Vec<Point>> = HashMap::new();
	for &cur_point in points.iter() {
		let mut hm: HashMap<i64, Vec<Point>> = HashMap::new();

		for &point in points.iter().filter(|&&a| a != cur_point) {
			let point_c = (point.0 - cur_point.0, point.1 - cur_point.1);

			let point_len = (point_c.0.powi(2) + point_c.1.powi(2)).sqrt();
			let cosine_angle = point_c.1 / point_len;
			let mut angle = cosine_angle.acos().to_degrees();
			if point_c.0 < 0.0 {
				angle += 180.0;
			}

			hm.entry((angle * 10_f64.powi(5)) as i64)
				.or_insert(Vec::new())
				.push(Point {
					point: point,
					len: (point_len * 10_f64.powi(5)) as i64,
				});
		}
		if hm.len() > best {
			best = hm.len();
			station_asteroids = hm;
		}
	}
	println!("Part 1: {}", best);

	let mut angles: Vec<(i64, Vec<Point>)> = station_asteroids.into_iter().collect();
	angles.sort_by_key(|e| e.0);
	angles.iter_mut().for_each(|v| v.1.sort_by_key(|p| p.len));
	let mut shot = 0;
	'loop_: loop {
		for angle in angles.iter_mut() {
			if angle.1.len() == 0 {
				continue;
			}
			let point = angle.1.remove(0);
			shot += 1;
			if shot == 200 {
				println!("Part 2: {}", point.point.0 * 100.0 + point.point.1);
				break 'loop_;
			}
		}
	}
}

struct Point {
	point: (f64, f64),
	len: i64,
}
