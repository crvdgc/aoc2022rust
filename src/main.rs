use std::env;
use std::fs;
mod d1;
mod d10;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;
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
        (2, 0) => d2::part0(input_str),
        (2, 1) => d2::part1(input_str),
        (3, 0) => d3::part0(input_str),
        (3, 1) => d3::part1(input_str),
        (4, 0) => d4::part0(input_str),
        (4, 1) => d4::part1(input_str),
        (5, 0) => d5::part0(input_str),
        (5, 1) => d5::part1(input_str),
        (6, 0) => d6::part0(input_str),
        (6, 1) => d6::part1(input_str),
        (7, 0) => d7::part0(input_str),
        (7, 1) => d7::part1(input_str),
        (8, 0) => d8::part0(input_str),
        (8, 1) => d8::part1(input_str),
        (9, 0) => d9::part0(input_str),
        (9, 1) => d9::part1(input_str),
        (10, 0) => d10::part0(input_str),
        (10, 1) => d10::part1(input_str),
        _ => panic!("Unimplemented: day {} part {}", day, part),
    }
}
