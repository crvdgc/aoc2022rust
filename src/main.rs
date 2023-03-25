use std::env;
use std::fs;
mod d1;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d2;
mod d20;
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
        (11, 0) => d11::part0(&input_str[..]),
        (11, 1) => d11::part1(&input_str[..]),
        (12, 0) => d12::part0(&input_str[..]),
        (12, 1) => d12::part1(&input_str[..]),
        (12, 2) => d12::part0(d12::example_input()),
        (12, 3) => d12::part1(d12::example_input()),
        (13, 0) => d13::part0(&input_str[..]),
        (13, 1) => d13::part1(&input_str[..]),
        (13, 2) => d13::part0(d13::example_input()),
        (13, 3) => d13::part1(d13::example_input()),
        (14, 0) => d14::part0(&input_str[..]),
        (14, 1) => d14::part1(&input_str[..]),
        (14, 2) => d14::part0(d14::example_input()),
        (14, 3) => d14::part1(d14::example_input()),
        (15, 0) => d15::part0(&input_str[..], 2000000),
        (15, 1) => d15::part1(&input_str[..], 4000000, 4000000),
        (15, 2) => d15::part0(d15::example_input(), 10),
        (15, 3) => d15::part1(d15::example_input(), 20, 20),
        (16, 0) => d16::part0(&input_str[..]),
        (16, 1) => d16::part1(&input_str[..]),
        (16, 2) => d16::part0(d16::example_input()),
        (16, 3) => d16::part1(d16::example_input()),
        (17, 0) => d17::part0(&input_str[..]),
        (17, 1) => d17::part1(&input_str[..]),
        (17, 2) => d17::part0(d17::example_input()),
        (17, 3) => d17::part1(d17::example_input()),
        (18, 0) => d18::part0(&input_str[..]),
        (18, 1) => d18::part1(&input_str[..]),
        (18, 2) => d18::part0(d18::example_input()),
        (18, 3) => d18::part1(d18::example_input()),
        (19, 0) => d19::part0(&input_str[..]),
        (19, 1) => d19::part1(&input_str[..]),
        (19, 2) => d19::part0(d19::example_input()),
        (19, 3) => d19::part1(d19::example_input()),
        (20, 0) => d20::part0(&input_str[..]),
        (20, 1) => d20::part1(&input_str[..]),
        (20, 2) => d20::part0(d20::example_input()),
        (20, 3) => d20::part1(d20::example_input()),
        _ => panic!("Unimplemented: day {} part {}", day, part),
    }
}
