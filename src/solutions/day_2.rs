pub mod solution {
    use crate::common::file_reader::reader;

	pub const INPUT_FILEPATH: &str = "./resources/day_2_input.txt";

	enum Choice {
		Rock,
		Paper,
		Scissors,
	}

	impl Choice {
		fn value(&self) -> u32 {
			match self {
				Choice::Rock => 1,
				Choice::Paper => 2,
				Choice::Scissors => 3,
			}
		}

		fn win(&self) -> Choice {
			match self {
				Choice::Rock => Choice::Scissors,
				Choice::Paper => Choice::Rock,
				Choice::Scissors => Choice::Paper,
			}
		}

		fn lose(&self) -> Choice {
			match self {
				Choice::Rock => Choice::Paper,
				Choice::Paper => Choice::Scissors,
				Choice::Scissors => Choice::Rock,
			}
		}

		fn draw(&self) -> Choice {
			match self {
				Choice::Rock => Choice::Rock,
				Choice::Paper => Choice::Paper,
				Choice::Scissors => Choice::Scissors,
			}
		}

		fn other_lose(&self) -> Choice {
			match self {
				Choice::Rock => Choice::Scissors,
				Choice::Paper => Choice::Rock,
				Choice::Scissors => Choice::Paper,
			}
		}

		fn other_win(&self) -> Choice {
			match self {
				Choice::Rock => Choice::Paper,
				Choice::Paper => Choice::Scissors,
				Choice::Scissors => Choice::Rock,
			}
		}

		// contest is evaluated from the perspective of the caller, i.e. self.
		// self is considered the winner or loser.
		fn evaluate_contest(&self, other: &Self) -> u32 {
			if self.win() == *other {
				6
			} else if self.lose() == *other {
				0
			} else { // self.draw() == other
				3
			}
		}

		fn parse(char: &str) -> Choice {
			match char {
				"A" | "X" => Choice::Rock,
				"B" | "Y" => Choice::Paper,
				"C" | "Z" => Choice::Scissors,
				_ => panic!("unmatched character: {}", char)
			}
		}

		fn parse_result_choice(&self, char: &str) -> Choice {
			match char {
				"X" => self.other_lose(),
				"Y" => self.draw(),
				"Z" => self.other_win(),
				_ => panic!("unmatched character: {}", char)
			}
		}
	}

	impl PartialEq for Choice {
    	fn eq(&self, other : &Self) -> bool {
    		match (self, other) {
    			(Choice::Rock, Choice::Rock) => true,
    			(Choice::Paper, Choice::Paper) => true,
    			(Choice::Scissors, Choice::Scissors) => true,
    			_ => false,
    		}
    	}
	}

	pub fn calculate_score(filepath: &str) -> Result<u32, Box<dyn std::error::Error>> {
		let lines = reader::read_lines(filepath)?;
		let mut result: u32 = 0;

		for line in lines {
			let choices = line?.clone();
			let mut choices = choices.split(" ");

			let other_choice = Choice::parse(choices.next().unwrap());
			let this_choice = Choice::parse(choices.next().unwrap());
			
			result = result + this_choice.value() + this_choice.evaluate_contest(&other_choice);
		}

		Ok(result)
	}

	pub fn calculate_score_part_2(filepath: &str) -> Result<u32, Box<dyn std::error::Error>> {
		let lines = reader::read_lines(filepath)?;
		let mut result: u32 = 0;

		for line in lines {
			let choices = line?.clone();
			let mut choices = choices.split(" ");

			let other_choice = Choice::parse(choices.next().unwrap());
			let this_choice = other_choice.parse_result_choice(choices.next().unwrap());
			
			result = result + this_choice.value() + this_choice.evaluate_contest(&other_choice);
		}

		Ok(result)
	}
}