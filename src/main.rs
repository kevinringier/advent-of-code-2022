mod common;
mod solutions;

use solutions::day_1;


fn main() {

    let file = day_1::solution::INPUT_FILEPATH;
    let result = day_1::solution::calculate_top_three_calories(file);

    if let Ok(sum) = result {
        println!("{}", sum);

    }
}
