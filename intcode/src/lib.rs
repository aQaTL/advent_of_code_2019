use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Default, Clone)]
pub struct Intcode {
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

	pub fn run_till_output(&mut self, input_queue: &[i64]) -> Option<i64> {
		let mut input_idx = 0;

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
					self.input.insert(param_1, input_queue[input_idx]);
					input_idx += 1;
					self.pc += 2;
				}
				4 => {
					let param_1 = self.get_param(1, &digits);
					self.pc += 2;
					return Some(param_1);
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
				9 if digits[1] == 9 => return None,
				_ => panic!(format!("unknown opcode {:?}", digits)),
			}
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
					let i1 = receiver.recv().unwrap();
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
