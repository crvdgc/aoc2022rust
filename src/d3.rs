fn priority(c: char) -> usize {
    let n = if c.is_ascii_lowercase() {
        c as usize - 'a' as usize
    } else if c.is_ascii_uppercase() {
        c as usize - 'A' as usize + 26
    } else {
        panic!("not valid char {}", c)
    };
    n
}

fn line_priority(line: &str) -> usize {
    let mut priority_map = vec![false; 26 + 26];
    let compartment_size = line.len() / 2;
    let first_compartment = &line[..compartment_size];
    let second_compartment = &line[compartment_size..];
    for c in first_compartment.chars() {
        priority_map[priority(c)] = true;
    }
    for c in second_compartment.chars() {
        let p = priority(c);
        if priority_map[p] {
            return p;
        }
    }
    panic!("no repeated item")
}
pub fn part0(input: String) -> () {
    // println!("{}, {}", priority('a'), priority('A'));
    let sum: usize = input.lines().map(line_priority).map(|x| x + 1).sum();
    println!("{}", sum)
}

fn update_line(line: &str, priority_map: &mut Vec<u32>) -> () {
    let mut seen = vec![false; 26 + 26];
    for c in line.chars() {
        seen[priority(c)] = true;
    }
    for (p, seen) in seen.iter().enumerate() {
        if *seen {
            priority_map[p] += 1
        }
    }
}

pub fn part1(input: String) -> () {
    let mut sum: usize = 0;
    for three_lines in input.lines().collect::<Vec<&str>>().chunks(3) {
        let mut priority_map: Vec<u32> = vec![0; 26 + 26];
        for line in three_lines {
            update_line(line, &mut priority_map)
        }
        let badge = priority_map.iter().position(|x| *x == 3).unwrap();
        sum += badge + 1;
    }
    println!("{}", sum)
}

pub fn example_input() -> String {
    r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#
    .to_string()
}
