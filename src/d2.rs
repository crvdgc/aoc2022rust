#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn fight_score(&self, other: &Shape) -> u32 {
        match (self, other) {
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Scissors, Shape::Scissors) => 3,
            (Shape::Rock, Shape::Scissors) => 6,
            (Shape::Paper, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Paper) => 6,
            _ => 0,
        }
    }

    fn from_result(&self, result: char) -> Self {
        match (result, self) {
            ('X', Shape::Rock) => Shape::Scissors,
            ('X', Shape::Paper) => Shape::Rock,
            ('X', Shape::Scissors) => Shape::Paper,
            ('Y', _) => self.clone(),
            ('Z', Shape::Rock) => Shape::Paper,
            ('Z', Shape::Paper) => Shape::Scissors,
            ('Z', Shape::Scissors) => Shape::Rock,
            _ => panic!("unknown result"),
        }
    }
}

fn line_to_score_part0(line: &str) -> u32 {
    let mut chars = line.chars();
    let opponent = match chars.next().unwrap() {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => panic!("unknown shape"),
    };
    let mine = match chars.skip(1).next().unwrap() {
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        _ => panic!("unknown shape"),
    };
    mine.score() + mine.fight_score(&opponent)
}

pub fn part0(input: String) -> () {
    let score: u32 = input.lines().map(line_to_score_part0).sum();
    println!("{}", score)
}

fn line_to_score_part1(line: &str) -> u32 {
    let mut chars = line.chars();
    let opponent = match chars.next().unwrap() {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => panic!("unknown shape"),
    };
    let result = chars.skip(1).next().unwrap();
    let mine = opponent.from_result(result);
    mine.score() + mine.fight_score(&opponent)
}

pub fn part1(input: String) -> () {
    let score: u32 = input.lines().map(line_to_score_part1).sum();
    println!("{}", score)
}

pub fn test_input() -> &'static str {
    r#"A Y
B X
C Z
"#
}
