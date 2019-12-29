use intcode::Intcode;
use std::sync::mpsc::channel;

macro_rules! spring_script {
	($sender:expr, {$($instr:tt $x:tt $y:tt)*}, $run_cmd:tt) => {
		concat!(
			$(concat!(stringify!($instr $x $y), '\n'),)*
			concat!(stringify!($run_cmd), '\n')
		)
		.bytes().for_each(|b| $sender.send(b as i64).unwrap());
	};
}

fn main() {
	let mut interpreter =
		Intcode::new(Intcode::parse_input(concat!(env!("CARGO_PKG_NAME"), "/input.txt")).unwrap());
	let mut interpreter_2 = interpreter.clone();

	let ((sender_p1, r), (s, receiver_p1)) = (channel(), channel());
	std::thread::spawn(move || interpreter.run(s, r));
	let ((sender_p2, r), (s, receiver_p2)) = (channel(), channel());
	std::thread::spawn(move || interpreter_2.run(s, r));

	spring_script!(
		&sender_p1,
		{
			NOT C J
			AND D J
			NOT A T
			OR T J
		},
		WALK
	);

	spring_script!(
		&sender_p2,
		{
			NOT B T
			NOT C J
			OR T J
			NOT A T
			OR T J
			AND D J
			OR E T
			OR H T
			AND T J
		},
		RUN
	);

	println!("Part 1: {}", receiver_p1.iter().max().unwrap());
	println!("Part 2: {}", receiver_p2.iter().max().unwrap());
}
