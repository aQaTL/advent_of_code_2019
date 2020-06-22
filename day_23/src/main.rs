use crossbeam_channel::{unbounded, Receiver, Sender};

struct Pipe {
	input: Sender<i64>,
	output: Receiver<i64>,
}

fn main() {
	let intcode = Intcode::new(Intcode::parse_input("day_23/input.txt").unwrap());
	let computers = vec![intcode; 50];
	let mut connections = Vec::with_capacity(computers.len());

	for (idx, mut computer) in computers.into_iter().enumerate() {
		let (sender, receiver) = unbounded();
		let (sender2, receiver2) = unbounded();

		computer.address = idx as i64;

		std::thread::spawn(move || computer.run(sender, receiver2));
		sender2.send(idx as i64).unwrap();

		connections.push(Pipe {
			input: sender2,
			output: receiver,
		});
	}

	let (nat_sender, nat_receiver) = unbounded();

	let mut nat_mem = vec![0; 2];

	crossbeam_utils::thread::scope(|scope| {
		let connections: &Vec<Pipe> = &connections;

		for idx in 0..50 {
			let nat_sender = nat_sender.clone();
			scope.spawn(move |_| {
				let mut buf = [0; 3];
				let mut buf_idx = 0;

				while let Ok(value) = connections[idx].output.recv() {
					buf[buf_idx] = value;

					if buf_idx != 0 {
						if buf[0] == 255 {
							nat_sender.send(value).unwrap();
						} else {
							connections[buf[0] as usize].input.send(value).unwrap();
						}
					}

					buf_idx = (buf_idx + 1) % buf.len();
				}
			});
		}

		let part_2 = scope
			.spawn(move |_| {
				let mut nat_history = HashSet::new();
				let mut count = 0;
				let mut received_first_nat_packet = false;

				loop {
					std::thread::sleep(std::time::Duration::from_millis(100));
					let all_empty = connections.iter().all(|conn| conn.output.is_empty());
					if all_empty {
						count += 1;
					}

					if count == 3 {
						let nat_vec = nat_receiver.try_iter().collect::<Vec<_>>();
						if !received_first_nat_packet && nat_vec.len() >= 2 {
							println!("Part 1: {}", nat_vec[1]);
							received_first_nat_packet = true;
						}

						if nat_vec.len() >= 2 {
							nat_mem = nat_vec[(nat_vec.len() - 2)..=(nat_vec.len() - 1)].to_vec();
						}
						connections[0].input.send(nat_mem[0]).unwrap();
						connections[0].input.send(nat_mem[1]).unwrap();
						if !nat_history.insert(nat_mem[1]) {
							return nat_mem[1];
						}
						count = 0;
					}
				}
			})
			.join()
			.unwrap();
		println!("Part 2: {}", part_2);
		exit(0);
	})
	.unwrap();
}

use std::collections::{HashMap, HashSet};
use std::io;
use std::path::Path;
use std::process::exit;

#[derive(Default, Clone)]
pub struct Intcode {
	address: i64,
	pc: i64,
	rel_base: i64,
	input: HashMap<i64, i64>,
}

impl Intcode {
	pub fn parse_input<P: AsRef<Path>>(path: P) -> io::Result<HashMap<i64, i64>> {
		let input = std::fs::read_to_string(path)?;
		let input = input
			.split(",")
			.enumerate()
			.map(|(idx, e)| (idx as i64, e.parse::<i64>().unwrap()))
			.collect::<HashMap<i64, i64>>();
		Ok(input)
	}

	pub fn new(input: HashMap<i64, i64>) -> Self {
		Intcode {
			input,
			..Default::default()
		}
	}

	fn get_digits(num: i64, count: u32) -> Vec<i64> {
		(0..count)
			.map(|digit| (num / 10_i64.pow(digit)) % 10)
			.collect::<Vec<i64>>()
	}

	fn get_param(&mut self, idx: i64, digits: &Vec<i64>) -> i64 {
		let mut inp = *self.input.entry(self.pc + idx).or_default();
		match digits[(idx + 1) as usize] {
			0 => inp = *self.input.entry(inp).or_default(),
			2 => inp = *self.input.entry(inp + self.rel_base).or_default(),
			_ => (),
		}
		inp
	}

	fn get_output(&mut self, idx: i64, digits: &Vec<i64>) -> i64 {
		let inp = *self.input.entry(self.pc + idx).or_default();
		if digits[(idx + 1) as usize] == 2 {
			inp + self.rel_base
		} else {
			inp
		}
	}

	pub fn run(&mut self, sender: Sender<i64>, receiver: Receiver<i64>) -> i64 {
		let mut prog_output: i64 = 0;

		loop {
			let digits = Self::get_digits(self.input[&self.pc], 5);
			match digits[0] {
				1 => {
					let output = self.get_output(3, &digits);
					let i = self.get_param(1, &digits) + self.get_param(2, &digits);
					self.input.insert(output, i);
					self.pc += 4;
				}
				2 => {
					let output = self.get_output(3, &digits);
					let i2 = self.get_param(1, &digits) * self.get_param(2, &digits);
					self.input.insert(output, i2);
					self.pc += 4;
				}
				3 => {
					let param_1 = self.get_output(1, &digits);
					let i1 = receiver.try_recv().unwrap_or(-1);
					self.input.insert(param_1, i1);
					self.pc += 2;
				}
				4 => {
					let param_1 = self.get_param(1, &digits);

					match sender.send(param_1) {
						_ => (),
					}
					prog_output = param_1;
					self.pc += 2;
				}
				5 => {
					let param_1 = self.get_param(1, &digits);
					let param_2 = self.get_param(2, &digits);
					if param_1 != 0 {
						self.pc = param_2;
					} else {
						self.pc += 3;
					}
				}
				6 => {
					let param_1 = self.get_param(1, &digits);
					let param_2 = self.get_param(2, &digits);
					if param_1 == 0 {
						self.pc = param_2;
					} else {
						self.pc += 3;
					}
				}
				7 => {
					let param_1 = self.get_param(1, &digits);
					let param_2 = self.get_param(2, &digits);
					let output = self.get_output(3, &digits);
					self.input
						.insert(output, if param_1 < param_2 { 1 } else { 0 });
					self.pc += 4;
				}
				8 => {
					let param_1 = self.get_param(1, &digits);
					let param_2 = self.get_param(2, &digits);
					let output = self.get_output(3, &digits);
					self.input
						.insert(output, if param_1 == param_2 { 1 } else { 0 });
					self.pc += 4;
				}
				9 if digits[1] != 9 => {
					let param_1 = self.get_param(1, &digits);
					self.rel_base += param_1;
					self.pc += 2;
				}
				9 if digits[1] == 9 => break,
				_ => panic!(format!("unknown opcode {:?}", digits)),
			}
		}

		prog_output
	}
}
