fn main() {
	let input = std::fs::read_to_string("day_8/input.txt")
		.unwrap()
		.trim()
		.bytes()
		.map(|c| c - 48)
		.collect::<Vec<_>>();

	let (width, height) = (25, 6);
	let layers = input.len() / (width * height);
	let mut img: Vec<&[u8]> = Vec::with_capacity(layers);

	(0..layers).for_each(|layer| {
		img.push(&input[(layer * width * height)..((layer + 1) * width * height)])
	});

	let fewest_0 = img
		.iter()
		.min_by_key(|layer| layer.iter().filter(|&&px| px == 0).count())
		.unwrap();
	let ones = fewest_0.iter().filter(|&&px| px == 1).count();
	let twos = fewest_0.iter().filter(|&&px| px == 2).count();
	println!("Part 1: {}", ones * twos);

	img.reverse();
	let mut final_img = vec![0u8; width * height];
	for layer in img {
		for (idx, &px) in layer.iter().enumerate() {
			if px != 2 {
				final_img[idx] = px;
			}
		}
	}

	println!("Part 2:");
	for j in 0..height {
		for i in 0..width {
			let px = final_img[j * width + i];
			if px == 1 {
				print!("\u{2588}");
			} else {
				print!(" ");
			}
		}
		println!();
	}
}
