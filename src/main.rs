mod common;
mod solutions;

use solutions::day_1;
use solutions::day_2;
use solutions::day_3;
use solutions::day_4;

fn main() {

    let file = day_4::solution::INPUT_FILEPATH;
    let result = day_4::solution::part_2(file);

    if let Ok(sum) = result {
        println!("{}", sum);
    }
}
