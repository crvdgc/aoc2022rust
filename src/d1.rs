pub fn part0(input: String) -> () {
    let max: Option<u32> = input
        .split("\n\n")
        .map(|s| s.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .max();
    if let Some(max) = max {
        println!("{}", max)
    } else {
        panic!("No max")
    }
}

pub fn part1(input: String) -> () {
    let mut sums: Vec<u32> = input
        .split("\n\n")
        .map(|s| s.lines().map(|line| line.parse::<u32>().unwrap()).sum())
        .collect();
    let mut top_3: u32 = 0;
    for _ in 0..3 {
        let max = sums
            .iter()
            .enumerate()
            .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
            .map(|x| x.clone());
        if let Some((index, max)) = max {
            top_3 += max;
            sums.remove(index);
        }
    }
    println!("{}", top_3)
}

pub fn test_input() -> &'static str {
    r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#
}
