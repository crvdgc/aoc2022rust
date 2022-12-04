fn contain_either(s1: (u32, u32), s2: (u32, u32)) -> bool {
    fn contain((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> bool {
        x1 <= x2 && y2 <= y1
    }
    contain(s1, s2) || contain(s2, s1)
}

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    fn parse_segment(s: &str) -> (u32, u32) {
        let mut iter = s.split("-").map(|x| x.parse::<u32>());
        let x = iter.next().unwrap().unwrap();
        let y = iter.next().unwrap().unwrap();
        (x, y) // assuming ordered
    }

    let mut iter = line.split(",").map(parse_segment);
    let s1 = iter.next().unwrap();
    let s2 = iter.next().unwrap();
    (s1, s2)
}

pub fn part0(input: String) -> () {
    let n_contained: u32 = input
        .lines()
        .map(parse_line)
        .map(|(s1, s2)| if contain_either(s1, s2) { 1 } else { 0 })
        .sum();
    println!("{}", n_contained)
}

fn overlap((x1, y1): (u32, u32), (x2, y2): (u32, u32)) -> bool {
    !(x1 < x2 && y1 < x2 || x1 > y2 && y1 > y2)
}

pub fn part1(input: String) -> () {
    let n_overlap: u32 = input
        .lines()
        .map(parse_line)
        .map(|(s1, s2)| if overlap(s1, s2) { 1 } else { 0 })
        .sum();
    println!("{}", n_overlap)
}

pub fn example_input() -> String {
    r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#
        .to_string()
}
