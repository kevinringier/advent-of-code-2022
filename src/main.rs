mod common;
mod solutions;

use solutions::day_1;
use solutions::day_2;


fn main() {

    let file = day_2::solution::INPUT_FILEPATH;
    let result = day_2::solution::calculate_score_part_2(file);

    if let Ok(sum) = result {
        println!("{}", sum);

    }


}
