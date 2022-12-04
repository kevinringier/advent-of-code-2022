pub mod solution {
	use crate::common::file_reader::reader;

	use std::ops::{Range, RangeInclusive};

	pub const INPUT_FILEPATH: &str = "./resources/day_4_input.txt";

	pub fn part_1(filepath: &str) -> Result<u32, Box<dyn std::error::Error>> {
		let lines = reader::read_lines(filepath)?;
		let mut result: u32 = 0;

		for line in lines {
			let l = line?;
			let mut split = l.split(",");
			let range_1 = split.next().unwrap();
			let range_2 = split.next().unwrap();

			let range_1 = parse_range(range_1);
			let range_2 = parse_range(range_2);

			if range_1.range_full_contains(&range_2) || range_2.range_full_contains(&range_1) {
				result = result + 1;
			}
		}

		Ok(result)
	}

	pub fn part_2(filepath: &str) -> Result<u32, Box<dyn std::error::Error>> {
		let lines = reader::read_lines(filepath)?;
		let mut result: u32 = 0;

		for line in lines {
			let l = line?;
			let mut split = l.split(",");
			let range_1 = split.next().unwrap();
			let range_2 = split.next().unwrap();

			let range_1 = parse_range(range_1);
			let range_2 = parse_range(range_2);

			if range_1.range_contains(&range_2) || range_2.range_contains(&range_1) {
				result = result + 1;
			}
		}

		Ok(result)
	}

	fn parse_range(range: &str) -> RangeInclusive<u32> {
		let mut split = range.split("-");
		let start = split.next().unwrap().parse::<u32>().unwrap();
		let end = split.next().unwrap().parse::<u32>().unwrap();

		start..=end
	}

	trait RangeContains<T> {
		fn range_full_contains(&self, other: &Self) -> bool;

		fn range_contains(&self, other: &Self) -> bool;
	}

	impl RangeContains<u32> for RangeInclusive<u32> {
		fn range_full_contains(&self, other: &Self) -> bool {
		    if self.contains(other.start()) && self.contains(other.end()) {
		    	true
		    } else {
		    	false
		    }
		}

		fn range_contains(&self, other: &Self) -> bool {
			if self.contains(other.start()) || self.contains(other.end()) {
		    	true
		    } else {
		    	false
		    }
		}
	}

	#[cfg(test)]
	mod tests {
		use crate::solutions::day_4::solution::RangeContains;

		#[test]
		fn test_range_contains_1() {
			let r_1 = 2..=10;
			let r_2 = 1..=11;

			let result = r_1.range_contains(&r_2);

			assert_eq!(result, false);
		}

		#[test]
		fn test_range_contains_2() {
			let r_1 = 0..=10;
			let r_2 = 1..=11;

			let result = r_1.range_contains(&r_2);

			assert_eq!(result, true);
		}

		#[test]
		fn test_range_contains_3() {
			let r_1 = 5..=12;
			let r_2 = 1..=11;

			let result = r_1.range_contains(&r_2);

			assert_eq!(result, true);
		}

		#[test]
		fn test_range_contains_4() {
			let r_1 = 1..=11;
			let r_2 = 1..=11;

			let result = r_1.range_contains(&r_2);

			assert_eq!(result, true);
		}

		#[test]
		fn test_range_contains_5() {
			let r_1 = 12..=14;
			let r_2 = 1..=11;

			let result = r_1.range_contains(&r_2);

			assert_eq!(result, false);
		}

		#[test]
		fn test_range_contains_6() {
			let r_1 = 0..=1;
			let r_2 = 1..=11;

			let result = r_1.range_contains(&r_2);

			assert_eq!(result, true);
		}

		#[test]
		fn test_range_contains_7() {
			let r_1 = 1..=4;
			let r_2 = 5..=11;

			let result = r_1.range_contains(&r_2);

			assert_eq!(result, false);
		}
	}
}