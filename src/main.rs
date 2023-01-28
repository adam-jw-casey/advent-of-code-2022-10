use std::env;
use std::fs;
use advent_of_code_2022_10::sum_strength;
use advent_of_code_2022_10::render;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read {file_path}");

    println!("The total strength is: {}", sum_strength(&contents));
    println!("{}", render(&contents));
}
