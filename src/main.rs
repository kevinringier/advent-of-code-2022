mod common;
mod solutions;

use solutions::day_1;
use solutions::day_2;
use solutions::day_3;


fn main() {

    let file = day_3::solution::INPUT_FILEPATH;
    let result = day_3::solution::part_2(file);

    if let Ok(sum) = result {
        println!("{}", sum);
    }
}
