pub mod solution {
	use crate::common::file_reader::reader;

	use std::{collections::HashSet, io::{Lines, BufReader}, fs::File};

	pub const INPUT_FILEPATH: &str = "./resources/day_3_input.txt";

	pub fn part_1(filepath: &str) -> Result<u32, Box<dyn std::error::Error>> {
		let lines = reader::read_lines(filepath)?;
		let mut duplicates: Vec<char> = Vec::new();

		for line in lines {
			let l = line?;
			let (compartment_1, compartment_2) = l.split_at(l.len() / 2); // assuming this is even and will work out.

			let mut compartment_1_set: HashSet<char> = std::collections::HashSet::new();

			for c in compartment_1.chars() { // should be valid ascii chars [a-z][A-Z]
				compartment_1_set.insert(c);
			}

			for c in compartment_2.chars() {
				if compartment_1_set.contains(&c) {
					duplicates.push(c);

					// we assume there is only one duplicate
					// additional duplicates shouldn't be calculated
					// so we break as soon as we match
					break;
				}
			}
		}

		Ok(duplicates.iter().fold(0, |acc, x| acc + get_char_value(&x)))
	}

	pub fn part_2(filepath: &str) -> Result<u32, Box<dyn std::error::Error>> {
		let mut lines = reader::read_lines(filepath)?;
		let mut duplicates: Vec<char> = Vec::new();

		let (group_1, group_2, group_3) = partition(&mut lines);

		for i in 0..group_1.len() {

			let mut set_1: HashSet<char> = HashSet::new();
			let mut set_2: HashSet<char> = HashSet::new();

			for c in group_1[i].chars() {
				set_1.insert(c);
			}

			for c in group_2[i].chars() {
				if set_1.contains(&c) {
					set_2.insert(c);
				}
			}

			for c in group_3[i].chars() {
				if set_2.contains(&c) {
					duplicates.push(c);
					break;
				}
			}
		}
		
		Ok(duplicates.iter().fold(0, |acc, x| acc + get_char_value(&x)))
	}

	// I'm curious if rust has partial functions we could generically apply here
	fn partition(lines: &mut Lines<BufReader<File>>) -> (Vec<String>, Vec<String>, Vec<String>) {
		let mut v_1: Vec<String> = Vec::new();
		let mut v_2: Vec<String> = Vec::new();
		let mut v_3: Vec<String> = Vec::new();

		let mut i: usize = 0;

		for line in lines.into_iter() {
			if i == 0 {
				v_1.push(line.unwrap());
				i = i + 1;
			} else if i == 1 {
				v_2.push(line.unwrap());
				i = i + 1;
			} else {
				v_3.push(line.unwrap());
				i = 0;
			}
		}

		(v_1, v_2, v_3)
	}

	fn get_char_value(c: &char) -> u32 {
		match c {
			// [a-z]
			'a' => 1, 'b' => 2,	'c' => 3,
			'd' => 4, 'e' => 5,	'f' => 6,
			'g' => 7, 'h' => 8,	'i' => 9,
			'j' => 10, 'k' => 11, 'l' => 12,
			'm' => 13, 'n' => 14, 'o' => 15,
			'p' => 16, 'q' => 17, 'r' => 18,
			's' => 19, 't' => 20, 'u' => 21,
			'v' => 22, 'w' => 23, 'x' => 24,
			'y' => 25, 'z' => 26,
			// [A-Z] 
			'A' => 27, 'B' => 28, 'C' => 29,
			'D' => 30, 'E' => 31, 'F' => 32,
			'G' => 33, 'H' => 34, 'I' => 35,
			'J' => 36, 'K' => 37, 'L' => 38,
			'M' => 39, 'N' => 40, 'O' => 41,
			'P' => 42, 'Q' => 43, 'R' => 44,
			'S' => 45, 'T' => 46, 'U' => 47,
			'V' => 48, 'W' => 49, 'X' => 50,
			'Y' => 51, 'Z' => 52,
			_ => panic!("not a valid char in [a-z][A-Z]")
		}
	}

}