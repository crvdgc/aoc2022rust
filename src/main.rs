use std::env;
use std::fs;
mod d1;
pub mod util;

fn input(day: u32) -> String {
    let path = format!("input/d{}.txt", day);
    fs::read_to_string(path).expect("File not found")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u32 = args[1].parse().unwrap();
    let part: u32 = args[2].parse().unwrap();
    // println!("Day {}, stage {}", day, stage);
    let input_str = input(day);
    match (day, part) {
        (1, 0) => d1::part0(input_str),
        (1, 1) => d1::part1(input_str),
        _ => panic!("Unimplemented: day {} part {}", day, part),
    }
}
