pub mod solution {
    use crate::common::file_reader::reader;

	pub const INPUT_FILEPATH: &str = "./resources/day_1_input.txt";

	/// part 1
	pub fn calculate_max_calories(filepath: &str) -> Result<u64, Box<dyn std::error::Error>> {
		let lines = reader::read_lines(filepath)?;
		let mut cur_calories: u64 = 0;
		let mut max_calories: u64 = 0;
		
		for line in lines {
			let l = line?;
			if !l.is_empty() {
				let calories = l.parse::<u64>()?;
				cur_calories = cur_calories + calories;				
			} else {
				max_calories = if cur_calories > max_calories {
					cur_calories
				} else {
					max_calories
				};
				cur_calories = 0;
			}
		}

		Ok(max_calories)
	}

	/// part 2
	pub fn calculate_top_three_calories(filepath: &str) -> Result<u64, Box<dyn std::error::Error>> {
		let lines = reader::read_lines(filepath)?;
		let mut cur_calories: u64 = 0;
		let mut max_calories: [u64; 3] = [0, 0, 0];
		
		for line in lines {
			let l = line?;
			if !l.is_empty() {
				let calories = l.parse::<u64>()?;
				cur_calories = cur_calories + calories;	
			} else {
				arr_cas(&mut max_calories, cur_calories);
				cur_calories = 0;
			}
		}



		Ok(max_calories.iter().sum())
	}

	fn arr_cas<const ARR_LENGTH: usize>(arr: &mut [u64; ARR_LENGTH], val: u64) {
		let mut i: usize = 0;
		let mut prev_val = arr[i];

		while i < ARR_LENGTH && val > arr[i] {
			let temp_val = arr[i];
			arr[i] = val;
			prev_val = temp_val;

			if i != 0 {
				arr[i - 1] = prev_val;
			}

			i = i + 1;
		}
	}

	#[cfg(test)]
	mod tests {
		use crate::solutions::day_1::solution::arr_cas;

		#[test]
		// test inserts beginning of arr
		fn test_arr_cas_1() {
			let mut arr = [2, 4, 5];
			let val = 3;

			arr_cas(&mut arr, val);

			assert_eq!(arr, [3, 4, 5]);
		}

		#[test]
		// test inserts in middle of arr
		fn test_arr_cas_2() {
			let mut arr = [2, 4, 6];
			let val = 5;

			arr_cas(&mut arr, val);

			assert_eq!(arr, [4, 5, 6]);
		}

		#[test]
		// test inserts at the end of arr
		fn test_arr_cas_3() {
			let mut arr = [2, 4, 6];
			let val = 7;

			arr_cas(&mut arr, val);

			assert_eq!(arr, [4, 6, 7]);
		}
	}
}

